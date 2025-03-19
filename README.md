# Dioxus 0.6.0 + tailwindcss v4.0.14  + daisyUI 5.0.6

A template for Dioxus in order to make using daisyUI with Rust much easier.

### Build

Enter folder and:

```
npm install
```

To see the changes made to the `css file(s)` live, we will need to do the following code (while in the root of the project):

```
npm run daisy
```

Now, while you can run this project with `cargo run`, you'll have a much easier time working with it if you use the tool for Dioxus projects, `dx`. To use it, you can build and run the app for the default platform with the following command:

```
dx serve
```

If you would like to specify a different platform, use the `--platform` argument. See `dx serve --help` for a list of possible values for the platform argument.