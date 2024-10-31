# Leptos Developer Tools

> [!WARNING]
> This library is very unstable.
> When there is a problem, refresh the page or re-open Developer Tools.

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

1. Open the extension page in google chrome.

- `chrome://extensions` in the url bar and press enter.

- Click on the three dots in the top right of the browser, then click "More tools" then click "Extensions".

2. Activate developer mode.

Turn on the switch on the top right of the page that says "Developer mode".

3. Load unpacked extension.
   
Click on the button on the top left of the page that says "Load unpacked".

Select the `extension/dist` directory. `Leptos Devtools` should appear in your extension manager dashboard.

### 4. Initialize the app

Add this crate to your project's `Cargo.toml` file.

```toml
leptos_devtools = { git = "https://github.com/luoxiaozero/leptos-devtools" }
```

Add `leptos_devtools::devtools!()` before the `mount_to_body` function in `main`.

Add the `tracing` feature to `leptos`.

```toml
leptos = { version = "0.7", features = ["csr", "tracing"] }
```

Open the `Leptos` devtools in the Developer Tools window.

## Resources

[Solid Devtools](https://github.com/thetarnav/solid-devtools)

[Vue Devtools](https://github.com/vuejs/devtools)