## [0.4.2](https://github.com/DevYukine/rustfoil/compare/v0.4.1...v0.4.2) (2021-02-14)


### Bug Fixes

* **index:** encode brackets ([2631cd2](https://github.com/DevYukine/rustfoil/commit/2631cd28bd7c58f4f763ba1cf026b988cfea3de2))



## [0.4.1](https://github.com/DevYukine/rustfoil/compare/v0.4.0...v0.4.1) (2021-02-14)


### Bug Fixes

* **index:** encode name like the whatwg spec requires ([a1d428e](https://github.com/DevYukine/rustfoil/commit/a1d428e5f895a8c06cac497a85427a24dbfa7215))



# [0.4.0](https://github.com/DevYukine/rustfoil/compare/v0.3.0...v0.4.0) (2020-12-12)


### Bug Fixes

* **output_index:** create_dir_all expects a folder ([f636e82](https://github.com/DevYukine/rustfoil/commit/f636e82ae5821f0388f1ebbae0f54fd26c63b829))
* check if public key is there before trying to access it ([e220810](https://github.com/DevYukine/rustfoil/commit/e2208109f8feee0ebf034b706eeae6f19b1a29c8))
* **output_index:** create all folders for output path if they don't exist ([4acd40e](https://github.com/DevYukine/rustfoil/commit/4acd40e9de90c5ffeb9f01db4493f73986800427))


### Features

* implement tinfoil index location ([488657a](https://github.com/DevYukine/rustfoil/commit/488657acf60d87fcecabeb38fa1892f30a1f5c3e))



# [0.3.0](https://github.com/DevYukine/rustfoil/compare/v0.2.0...v0.3.0) (2020-12-08)


### Features

* **gdrive:** support google drive shortcuts ([5e8c025](https://github.com/DevYukine/rustfoil/commit/5e8c02557abe9a0a0fd22ac60f837176adc3781b))



# [0.2.0](https://github.com/DevYukine/rustfoil/compare/28b5b9a9f89b475c6f52535d604cfc4b7653e654...v0.2.0) (2020-12-07)


### Bug Fixes

* **compression:** remove duplicated compression ([6810519](https://github.com/DevYukine/rustfoil/commit/681051909d5bc8c6fa524b66cca2a6f4e1a0a670))
* **gdrive:** fix sharing not working correctly ([22d0a32](https://github.com/DevYukine/rustfoil/commit/22d0a32dbdc44abfb9ea3769c28995b0204c81d3))
* **gdrive:** fix typo ([a7aa40c](https://github.com/DevYukine/rustfoil/commit/a7aa40cbb7bdf783899185b4e7462a4c8ab4bf31))
* **gdrive:** fix upload to specific folder id ([730667b](https://github.com/DevYukine/rustfoil/commit/730667b744c76df3e4ad67fb101e0cd77d246d30))
* **gdrive:** rename vector to vec ([6e2095c](https://github.com/DevYukine/rustfoil/commit/6e2095c257ba789c3a421f28309984d398d7e9f8))
* **gdrive:** support all drives while deleting old permissions ([40817f7](https://github.com/DevYukine/rustfoil/commit/40817f7b9c17263c9ef945dec0ff982c8f106a4d))
* **index:** correct themeWhiteList and themeBlackList json key names ([#6](https://github.com/DevYukine/rustfoil/issues/6)) ([97cb61e](https://github.com/DevYukine/rustfoil/commit/97cb61ecbf846058cbdc725e926eb1521c3856e7))
* **index:** serialize all fields in camelCase ([#4](https://github.com/DevYukine/rustfoil/issues/4)) ([2143f97](https://github.com/DevYukine/rustfoil/commit/2143f97df3c266602ce5f5ca68189496cc45feb7))
* **logging:** only display debug & trace if verbose is enabled ([5f8c065](https://github.com/DevYukine/rustfoil/commit/5f8c065b161fa53aae6a618a9a49319d4dd760ad))
* **main:** catch logging result ([e3a1045](https://github.com/DevYukine/rustfoil/commit/e3a104538df945e2bda0dd0e0204f9c498e9eb68))
* **main:** correct default output file extension ([c86db37](https://github.com/DevYukine/rustfoil/commit/c86db3704c32b9c93c6efc7271af2353f89c02b0))
* **main:** display the possible values of --compression option ([#5](https://github.com/DevYukine/rustfoil/issues/5)) ([9d65589](https://github.com/DevYukine/rustfoil/commit/9d6558972591629512ae84586b688e890a9c7432))
* **main:** use reference to not take ownership ([7aee585](https://github.com/DevYukine/rustfoil/commit/7aee585ba5eab81f49dd452bf1da4b7ba43102cd))
* borrow folder_id ([847678f](https://github.com/DevYukine/rustfoil/commit/847678f8aa4b702c44cfe752f7d5c1e1da3185b8))
* encrypt extra bytes  ([7bb525b](https://github.com/DevYukine/rustfoil/commit/7bb525b677e8f231e075ac3130ef5c082a077bd7))
* enum's should be passed by value ([98cf267](https://github.com/DevYukine/rustfoil/commit/98cf2677da261cbebaf00a37f5436cb758c039f8))
* enum's should implement copy ([ee22cd2](https://github.com/DevYukine/rustfoil/commit/ee22cd2cfcbe048ffa46c993cdf2a2b1e0fd97cc))
* tinfoil minimum version should be a float ([8f54c32](https://github.com/DevYukine/rustfoil/commit/8f54c329699b52937251ced44e016d595b849afb))
* **main:** progress bar is not a download ([5da75bc](https://github.com/DevYukine/rustfoil/commit/5da75bcb3e0d444b5ffa7cd3c352bf803d4365e6))
* **main:** use correct loglevel ([ba5cfd7](https://github.com/DevYukine/rustfoil/commit/ba5cfd77878d6c5c95f9eb365abf1e0dcde53832))
* remove dead imports ([28b5b9a](https://github.com/DevYukine/rustfoil/commit/28b5b9a9f89b475c6f52535d604cfc4b7653e654))


### Features

* add share folder flag ([35af18e](https://github.com/DevYukine/rustfoil/commit/35af18ece11bfe29219b3ef5c3966d4aa952ebbd))
* add tinfoil auth flag ([de9092f](https://github.com/DevYukine/rustfoil/commit/de9092f2aac3438dddb9f6ea9f963b85b2c71932))
* **main:** output error correctly ([02d8a02](https://github.com/DevYukine/rustfoil/commit/02d8a0287ae0c98810f73056d1cbbbc331f9b354))
* **main:** output url of index when shared ([12a6092](https://github.com/DevYukine/rustfoil/commit/12a6092c7d749cb0c7a281ce5b20e26bf8e4eec5))
* **main:** replace newline and tab characters ([d8bb868](https://github.com/DevYukine/rustfoil/commit/d8bb86817030cbf38dbe42ffb0b0e4b0172546fc))
* add basic compression ([9f43aa6](https://github.com/DevYukine/rustfoil/commit/9f43aa647931c6e286b92d51a6a80c585660b66f))
* add encryption ([097e8f0](https://github.com/DevYukine/rustfoil/commit/097e8f0a0582513d46452ff8417dbbe084f5c889))
* add headless mode ([7ce8ecb](https://github.com/DevYukine/rustfoil/commit/7ce8ecb4a578c50a7fbb9b5d7eb55e4f76f1e606))
* add index upload & sharing ([3a57684](https://github.com/DevYukine/rustfoil/commit/3a5768498932b3f92edecd5c076268b4e7de2105))
* add openssl building deps ([35a93d7](https://github.com/DevYukine/rustfoil/commit/35a93d7dc9ffb64daf354c3c68284879a6318502))
* allow to add multiple folder ids ([fd05a0f](https://github.com/DevYukine/rustfoil/commit/fd05a0f0b5c203ee38685e16bac679446dd39216))
* **compression:** add Display impl for CompressionFlag ([89cb1f1](https://github.com/DevYukine/rustfoil/commit/89cb1f18a3ea4e1e05e3b157bfd537b56e4eb395))
* **compression:** catch possible error of encoder#write_all ([28f7b7e](https://github.com/DevYukine/rustfoil/commit/28f7b7eb83e5dfce25381a9945c13675e5a8bd33))
* **main:** log compression if used ([326e4e8](https://github.com/DevYukine/rustfoil/commit/326e4e817bf508ee743361051d7068b00fe6f6ed))
* add sharing for files ([f5f6fb1](https://github.com/DevYukine/rustfoil/commit/f5f6fb1667dde36ad4447e2dfe16cb4c1d9a69bb))
* add support for more tinfoil index features ([6c01d0e](https://github.com/DevYukine/rustfoil/commit/6c01d0e7848f23e66b4481e5f2b83f87b8332eec))
* convert output file to tinfoil format ([eb1b20f](https://github.com/DevYukine/rustfoil/commit/eb1b20f6d492a01ddb7e6dd53106bc68bd813635))
* **index:** add ParsedFileInfo ([9ba9f44](https://github.com/DevYukine/rustfoil/commit/9ba9f44d72a683c21599da3e8c111598b5f1a005))
* **index:** ignore empty success message ([e32ed9d](https://github.com/DevYukine/rustfoil/commit/e32ed9d982075cc46c378fb623034df6b1931599))
* **main:** write index to disk, add title id & file extension checks ([e3d0d0a](https://github.com/DevYukine/rustfoil/commit/e3d0d0aaa4419aa940b6a265d14fd4e8f6c9f3bf))



