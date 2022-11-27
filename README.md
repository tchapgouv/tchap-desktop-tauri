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
```
curl https://github.com/tchapgouv/tchap-web-v4/releases/download/tchap-4.1.0_1.11.10/tchap-4.1.0_1.11.10-prod-20221107.tar.gz
tar -xvf tchap-4.1.0_1.11.10-prod-20221107.tar.gz
mv dist src
```
- Compiler tchap pour votre système
```
cd src-tauri
cargo tauri build
```

### Problèmes

Avec l'app en AppImage :
- Impossible de remettre l'application en avant dans certain cas à partir de la tray icon
- Pas de son/video
- Pas de téléchargement
- Pas d'upload de fichier
