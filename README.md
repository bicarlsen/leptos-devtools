# Leptos Developer Tools

> [!WARNING]
> This library is very unstable.
> When there is a problem, refresh the page or re-open Developer Tools, and please [raise an Issue on GitHub](https://github.com/luoxiaozero/leptos-devtools/issues).

> [!NOTE]
> Currenlty only available for Google Chrome.

## Install

### 1. Clone the repository

```sh
git clone git@github.com:luoxiaozero/leptos-devtools.git
```

### 2. Build the extension

Execute the following command in the `extension` directory.

```sh
pnpm install

pnpm build
```

### 3. Install the Chrome extension

1. [Open the extension page](https://support.google.com/chrome_webstore/answer/2664769?hl=en) in Google Chrome.

- Go to the URL `chrome://extensions`.

- Open the kebab menu (three dots in the top right of the browser), then navigate to "Extensions" > "Manage Extensions".

2. Activate developer mode.

    Turn on the switch on the top right of the page that says "Developer mode".

3. Load unpacked extension.
   
    Click on the button on the top left of the page that says "Load unpacked".

    Select the `extension/dist` directory. `Leptos Devtools` should appear in your extension manager dashboard.

### 4. Initialize the app

1. Add the `tracing` feature to `leptos`.

    ```toml
    leptos = { version = "0.7", features = ["csr", "tracing"] }
    ```

2. Add this crate to your project's `Cargo.toml` file.

    ```toml
    leptos_devtools = { git = "https://github.com/luoxiaozero/leptos-devtools" }
    ```

3. Initialize the devtools.
    
    Add `leptos_devtools::devtools!()` before the `mount_to_body` function in `main`.

4. Register the devtools `tracing` layer.

    ```rust
    use tracing_subscriber::prelude::*;
    use leptos_devtools::Devtools;

    tracing_subscriber::registry()
        .with(Devtools::default())
        .init();
    ```

5. Open the `Leptos` devtools in the Developer Tools window.

## Resources

[Solid Devtools](https://github.com/thetarnav/solid-devtools)

[Vue Devtools](https://github.com/vuejs/devtools)