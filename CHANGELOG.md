# [1.0.0](https://github.com/DevYukine/rustfoil/compare/v0.5.1...v1.0.0) (2023-11-21)


* feat!: add missing tinfoil keys back to CLI, require them in lib ([fbe5c13](https://github.com/DevYukine/rustfoil/commit/fbe5c1318d83c2eef39527a994865ee9cf31bcdd))


### BREAKING CHANGES

* changes parameter of TinfoilService#generate_index method in lib

This also adds googleApiKey, headers back as CLI flags



## [0.5.1](https://github.com/DevYukine/rustfoil/compare/v0.5.0...v0.5.1) (2023-11-19)


### Bug Fixes

* **ci:** correct actions/checkout version in release action ([8f6d570](https://github.com/DevYukine/rustfoil/commit/8f6d570d4047ae57036244596c77b684f177a3ac))



# [0.5.0](https://github.com/DevYukine/rustfoil/compare/v0.4.2...v0.5.0) (2023-11-19)


### Bug Fixes

* **http:** only url encode the filename, not other parts of the path ([6d79ec1](https://github.com/DevYukine/rustfoil/commit/6d79ec1f50dd79a65d29bd9694d68cf4bf8bd04b))
* **http:** only url encode the filename, not other parts of the path ([64d0310](https://github.com/DevYukine/rustfoil/commit/64d0310100da519488f1ec35f7c78dd3f1e00941))
* runtime error ([3e9faa8](https://github.com/DevYukine/rustfoil/commit/3e9faa8636911b35b39840e383c40eaecd41235e))


### Features

* rewrite and add support for local files via http/https ([2795675](https://github.com/DevYukine/rustfoil/commit/2795675113b12bc330472d1f6995f4a996e19198))



## [0.4.2](https://github.com/DevYukine/rustfoil/compare/v0.4.1...v0.4.2) (2021-02-14)


### Bug Fixes

* **index:** encode brackets ([2631cd2](https://github.com/DevYukine/rustfoil/commit/2631cd28bd7c58f4f763ba1cf026b988cfea3de2))



## [0.4.1](https://github.com/DevYukine/rustfoil/compare/v0.4.0...v0.4.1) (2021-02-14)


### Bug Fixes

* **index:** encode name like the whatwg spec requires ([a1d428e](https://github.com/DevYukine/rustfoil/commit/a1d428e5f895a8c06cac497a85427a24dbfa7215))



