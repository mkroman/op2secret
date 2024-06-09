# op2secret

This is a simple CLI tool leverages the [1Password CLI] to generate a Kubernetes
Secret manifest from a 1Password secret.

It was made to make my life easier when updating the sealed secrets in my
Kubernetes cluster.

[1Password CLI]: https://developer.1password.com/docs/cli/get-started/

## Details

The tool uses the [1Password CLI] to read an item and its fields from 1Password.

The name of the generated secret manifest will be derived from the title of the
1Password secret unless specified in the `name` argument.

If the secrets title is in the format `<namespace>/<name>`, then the namespace
will also be derived.

Fields that are references to other fields or secrets are ignored. The notes
portion of the secret is technically also a field, but this is ignored.

## Usage

### Reference

```
Usage: op2secret <reference> [<name>] [-t <type>] [-n <namespace>] [--op-bin <op-bin>]

Generate a Kubernetes Secrets manifest from a 1Password secret.

Positional Arguments:
  reference         reference to 1password secret
  name              optional name of the kubernetes secret. If not specified,
                    the name will be derived from the secret title. Also see the
                    description for `namespace'

Options:
  -t, --type        type of the kubernetes secret (defaults to `Opaque')
  -n, --namespace   namespace of the kubernetes secret - if not specified, and
                    the 1Password secret has a title in the format
                    `<namespace>/<name>`, then the namespace will be derived
                    from that
  --op-bin          path to 1Password CLI executable (overrides the OP_BIN
                    environment variable and defaults to `op')
  --help            display usage information
```

### Generate a basic auth secret

To read an item from 1Password, with a username and password, and generate a
secret with the type `kubernetes.io/basic-auth`:

```sh
op2secret -t kubernetes.io/basic-auth some-namespace/some-secret
```

This will generate the following manifest:

```yaml
apiVersion: v1
kind: Secret
metadata:
  name: some-secret
  namespace: some-namespace
stringData:
  password: bar
  username: foo
type: kubernetes.io/basic-auth
```

### Generate a sealed secret

To generate a bitnami sealed secret manifest instead of a regular secret, we can
pipe the manifest directly to `kubeseal`:

```sh
op2secret -t kubernetes.io/basic-auth some-namespace/some-secret | kubeseal -o yaml
```

This will result in something similar to:

```
---
apiVersion: bitnami.com/v1alpha1
kind: SealedSecret
metadata:
  creationTimestamp: null
  name: some-secret
  namespace: some-namespace
spec:
  encryptedData:
    password: AgBVXNveLww8ZSL72V9Iwe3T8mDwERWRD/3fpKbhsyiBVMjqtqz3DGlGu6y1gE8KOObwXQ/xD6nfhs/Gc3sRtwftIjwkwOjV39WE0ACIBsGPpXrmGDqsCDpGl4WM/LxiXT1N7E/CZqcc5jrSBoto70bxkaeuSm8h+rYPSjkt4s6oHEJoESUsnBj1eewU5s0Lw2WfPGA6nciXA6SUp4hhqbFW0vQNLg/Apu47rxcR4Xqzn+1E2jgFK77adGvaOoU9SzYEmdunMH1gE9r4f/AqzzfSe4NpPRpKFlwKvLbv7u+7uOHQgV9zErME2OdHt031lwSWE8wkZEOT0Xke5E3wOCDtC8RfrspVtKRMik5i17oz2YvNpVzX8uqXT/PBv4G3pkPFMw+uOP9epxyLBobbC99A8Keq/Rhe4Spa9d6/gpHJe07g1rZSRqSb1v3+Ro75x1n/0rfxDfOcg1+sc1BWm5b15d9NsCqWWN4gxPBgas80dQndmqroPKZO4PgEKyxsmfdedA+WGZsy9ynAqMmyJi5MIhQb9a3vkNQoQ6/ttzMKc1aNjzdciJifzeYunhK29/eL1E9g0K56rOl2gzWenH25joTmKmC13UdiX9mlV38SmFScT4GMYBLR960WcM5Oys5aF6Ej1E18XEuUOXxwNJF5hCqysWxgKomNyg4DYwrZIEGScsj50R4GR3hwuBCjnwPs+2k=
    username: AgCGTB18VTunR67fyrtZAySnIgf0SXvd2qiWEXRHCEcSl91E1dZdiC820/GySlmHOV+JHNTRf5SrVVpXtp0IHdoIvo9tzV1zVnfdMwJCOmy3oC9PeJVmdFD7/bRa6+YBTyq0JYXarqLsc24AMOSuVbKbidvmbyAH/VNlOUvrQK1fdgYKS+CRbbhTjnHbhhTXAxVy6DJ4lZHXo6qbt//Iu19qz5NtmFWuuvxTB+lpRZJfjriljq9sCf/gAiLyhre/YC7IvBAIWfMzRosbTgbpqStxQSURkW/3Z+kH4UG9mqXMA8ubjTwN3t0H6iie2OOhPyWBl7YNX03PQX/XOh+aIA625hJPsOvTWoS/Vhw0bWpZiAiyA0djBnGjl7zGmLEllvuXguvxCY1Ebcof41eSNJz6AlbbpwNotQ/oD/omz3YdP9Fx3hgXRLZvSVwNV1UMFOJLm08oGWNFAa5Tl7/ZtYhqSNDldlPqO/aw41Rwm7ifr7JEdq7RO4ViFtqt1X6b73T/qfvuOj7NMEkDaVx16iuWapelGG5jmREXu/PT43ewxCt7l1JcZfUeaJ6cYvnk0CZNVNawYtbT9p9kInxCcjC3KtXNhaGZPfcG+PGHrnqIQcabKmNBNUsps5vWJeWCMZPp/HVwrU95I71pUlwL+R1xcQnemOS2zdgcf/7IbqBlcnMaqaVgY77aIvswekvHXP5bjDo=
  template:
    metadata:
      creationTimestamp: null
      name: some-secret
      namespace: some-namespace
    type: kubernetes.io/basic-auth
```

## License

This software is dual-licensed under MIT or Apache 2.0, at your option.
