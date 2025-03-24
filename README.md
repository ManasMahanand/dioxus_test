# TODO App

A basic todo app for testing out dioxus.

### Tailwind
1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
pnpx @tailwindcss/cli -i ./input.css -o ./assets/tailwind.css --watch
```

### Serving The App

Run the following command in the root of the project to start developing with the default platform:

```bash
dx serve
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

