# Rustfoil

This CLI allows you to easy generate an index file for use with Tinfoil.

This project is based on [TinGen](https://github.com/eXhumer/TinGen) by [eXhumer](https://github.com/eXhumer) & [tinfoil_gdrive_generator](https://github.com/BigBrainAFK/tinfoil_gdrive_generator/) by [BigBrainAFK](https://github.com/BigBrainAFK) 

## Why

- Rust allows to bundle the complete application, no dependency installation required!
- I wanted to get back to rust again and this was a good project to take on!

## Requirements

- credentials.json (you can modify location & name with `--credentials` flag) It can be obtained from [here](https://developers.google.com/drive/api/v3/quickstart/python) by clicking the Enable Drive API button in there while being signed in with the user account you want to generate credentials for or from Google's Developer Console.
- Google Drive Folder IDs to scan and index

## Features

### Index

- [x] Generate index
- [x] Add success message
- [x] Change index name
- [x] Change output location

### Compression

- [x] Zlib
- [x] Zstd

### Encryption

- [ ] Encrypt with AES-ECB-256

### Upload 

- [ ] Upload index to own gdrive
- [ ] Upload index to team drive

### Share Files

- [ ] Share files inside index
- [ ] Share uploaded index

### Error Handling

- [ ] Retry gdrive exceptions
