# Rustfoil

This CLI allows you to easy generate an index file for use with Tinfoil.

This project is based on [TinGen](https://github.com/eXhumer/TinGen) by [eXhumer](https://github.com/eXhumer) & [tinfoil_gdrive_generator](https://github.com/BigBrainAFK/tinfoil_gdrive_generator/) by [BigBrainAFK](https://github.com/BigBrainAFK) 

## Why

- Rust allows to bundle the complete application, no dependency installation required!
- I wanted to get back to rust again and this was a good project to take on!

## Requirements

- credentials.json (you can modify location & name with `--credentials` flag) It can be obtained from [here](https://developers.google.com/drive/api/v3/quickstart/python) by clicking the Enable Drive API button in there while being signed in with the user account you want to generate credentials for or from Google's Developer Console.
- Google Drive Folder IDs to scan and index

## Usage

**NOTE:** the first time `rustfoil.exe` is ran, a URL will be displayed. Copy and paste that URL into your web browser and follow the instructions on screen to continue. This process generates a `token.json` file that allows rustfoil to access your drive.

- To use rustfoil to generate an `index.tfl` file corresponding to your Google Drive folder, run the following command:

```
rustfoil.exe GOOGLE_DRIVE_FOLDER_ID
```

This will generate an `index.tfl` file in the same directory that `rustfoil.exe` is located.

- Other flags and options:

Flag | Description
------------ | -------------
FLAGS
`--add-non-nsw-files` | Adds files without valid NSW ROM extension(NSP/NSZ/XCI/XCZ) to index
`--add-nsw-files-without-title-id` | Adds files without valid Title ID
`-h`, `--help` | Prints help information
`--headless` | If OAuth should be done headless
`--no-recursion` | Scans for files only in top directory for each Folder ID entered
`--share-files` | Share all files inside the index file (not recommended, use OAuth)
`--share-folders` | Share all folders inside the provided folders (not recommended, use OAuth)
`--share-index` | Shares the index file that is uploaded to Google Drive
`--tinfoil-auth` | If Tinfoil authentication files should be generated
`--upload-my-drive` | If the index file should be uploaded to My Drive
`-V`, `--version` | Prints version information
`-v`, `--verbose` | Verbose mode (`-v`, `-vv`, `-vvv`, etc.)
OPTIONS
`--compression <compression>` | Which compression should be used for the index file [default: zstd]  [possible values: Off, ZSTD, Zlib]
`--credentials <credentials>` | Path to Google Application Credentials [default: credentials.json]
`--google-api-key <google-api-key>` | Adds a google API key to be used with all gdrive:/ requests
`--headers <headers>...` | Adds custom HTTP headers Tinfoil should send with its requests
`--location-path <location-path>` | Path to location.json file
`--min-version <min-version>` | Adds a minimum Tinfoil version to load the index
`--one-fichier-keys <one-fichier-keys>...` | Adds 1Fincher API keys to be used with all 1f:/ requests, If multiple keys are provided, Tinfoil keeps trying them until it finds one that works
`-o <output-path>` or `--output-path <output-path>` | Path to output index file [default: index.tfl]
`--public-key <public-key>` | Path to RSA Public Key to encrypt AES-ECB-256 key with
`--referrer <referrer>` | Adds a referrer to index file to prevent others from hotlinking
`--success <success>` | Adds a success message to index file to show if index is successfully read by Tinfoil
`--theme-blacklist <theme-blacklist>...` | Adds a list of themes to blacklist based on their hash
`--theme-error <theme-error>` | Adds a custom theme error message to the index
`--theme-whitelist <theme-whitelist>...` | Adds a list of themes to whitelist based on their hash
`--tinfoil-auth-path <tinfoil-auth-path>` | Path to Tinfoil authentication files [default: COPY_TO_SD/switch/tinfoil]
`--token <token>` | Path to Google OAuth2.0 User Token [default: token.json]
`--upload-folder-id <upload-folder-id>` | If the index file should be uploaded to specific folder

## (Planned) Features

### Index

- [x] Generate index (full spec support)
- [x] Change index name
- [x] Change output location

### Compression

- [x] Zlib
- [x] Zstd

### Encryption

- [x] Allow to use Tinfoil encryption (DRM Spec)

### Upload 

- [x] Upload index to own gdrive
- [x] Upload index to team drive

### Sharing

- [x] Share files inside index
- [x] Share folders
- [x] Share uploaded index

### Error Handling

- [ ] Retry gdrive exceptions
