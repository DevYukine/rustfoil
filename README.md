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

- To use **rustfoil** to generate an `index.tfl` file corresponding to your Google Drive folder, run the following command:

```
rustfoil.exe GOOGLE_DRIVE_FOLDER_ID
```

This will generate an `index.tfl` file in the same directory that `rustfoil.exe` is located.

**NOTE:** the first time `rustfoil.exe` is run, a URL will be displayed. Simply follow the instructions on screen to continue.

- To specify a location in which your `credentials.json` file is located, add the `--credentials` flag as shown:

```
rustfoil.exe GOOGLE_DRIVE_FOLDER_ID --credentials "PATH/TO/CREDENTIALS/credentials.json"
```

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
