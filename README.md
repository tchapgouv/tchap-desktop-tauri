# Tchap Desktop

Expérimentation d'un client Desktop Tchap avec Tauri ( https://tauri.app )

## Comment compiler 

### Sous une distribution debian

- Installation de dépendance 
```
apt install libwebkit2gtk-4.0-dev     build-essential     curl     wget     libssl-dev     libgtk-3-dev     libayatana-appindicator3-dev     librsvg2-dev
```
- Installation de Rust
```
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y
source "$HOME/.cargo/env
```
- Télécharger la CLI tauro`
```
cargo install tauri-cli
```
- Télécharger une release de Tchap Web et placer le contenu de dist dans src
- Compiler tchap pour votre système
```
cd src-tauri
cargo tauri build
```


