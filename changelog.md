<a name="v1.4.0"></a>
## v1.4.0 (2019-08-21)

#### Breaking Changes
* **util:** Added header bool param to util::csv::serialize_to_csv_str method.
* **streams:** upload_serializable_part now returns an error if an empty array is passed in.
* **datasets:** upload_serializable now returns an error if an empty array is passed in.

#### Chore

*   update docs link to latest version ([4a03d409](https://github.com/quantumZebraPDX/domo-pitchfork/commit/4a03d409b924e61ff91600e888df27512740ecdb))



<a name="v1.3.1"></a>
### v1.3.1 (2019-07-31)


#### Bug Fixes

*   fixed tls feature to allow rustls to be used ([e5f7ed8a](https://github.com/quantumZebraPDX/domo-pitchfork/commit/e5f7ed8a42bf33ff61de94ab6848eb769f683881))



<a name="v1.3.0"></a>
## v1.3.0 (2019-07-24)


#### Breaking Changes

* **users:**  Fix user & group structs to match API ([2f071fc9](https://github.com/quantumZebraPDX/domo-pitchfork/commit/2f071fc996ba17471e3fdb1dc595c4a8a533aa8f)

#### Chore

* **clippy:**  applied a few clippy fixes ([09eedce0](https://github.com/quantumZebraPDX/domo-pitchfork/commit/09eedce07ac33b953edd2ffe457eebed37bf27db))
* **deps:**  updated csv crate version ([bdc790ff](https://github.com/quantumZebraPDX/domo-pitchfork/commit/bdc790ffb7fa1ced42d40612ab3ca01e26fa2de9))

#### Features

*   Auth scope builder ([d28c9906](https://github.com/quantumZebraPDX/domo-pitchfork/commit/d28c99067fcda8119778cf060c58f8b3bd60ab31))
* **datasets:**  add Date/DateTime schema support ([12b37788](https://github.com/quantumZebraPDX/domo-pitchfork/commit/12b37788899b8434dcb9d49daa51e77c95fe07e5))

#### Bug Fixes

* **datasets:**
  *  properly merge TDate and TDateTime ([b8fceee6](https://github.com/quantumZebraPDX/domo-pitchfork/commit/b8fceee6c9ac843349cee338d86c071e9e216ef1))
  *  make from_fieldtype public ([5ce44992](https://github.com/quantumZebraPDX/domo-pitchfork/commit/5ce44992c032296d8adfebcf7e791b4fd3cbf227))
* **users:**  Fix user & group structs to match API ([2f071fc9](https://github.com/quantumZebraPDX/domo-pitchfork/commit/2f071fc996ba17471e3fdb1dc595c4a8a533aa8f)



<a name="v1.2.1"></a>
### v1.2.1 (2019-05-06)


#### Documentation

*   fix README example and add doctest ([07a16359](https://github.com/quantumZebraPDX/domo-pitchfork/commit/07a1635933ae14873884bc569cfb851c37b77758))

#### Features

* **datasets:**  add csv deserialization ([86bfb9de](https://github.com/quantumZebraPDX/domo-pitchfork/commit/86bfb9de2ddbbe80d979bccf4488566e7d0d9bda))



<a name="v1.2.0"></a>
## v1.2.0 (2019-05-02)


#### Features

*   add cargo features for tls options ([67acf253](https://github.com/quantumZebraPDX/domo-pitchfork/commit/67acf253f37d1b1d6069e87c24ffe2b468124947), breaks [#](https://github.com/quantumZebraPDX/domo-pitchfork/issues/))

#### Breaking Changes

*   removed util::common and util::unstable mods ([67acf253](https://github.com/quantumZebraPDX/domo-pitchfork/commit/67acf253f37d1b1d6069e87c24ffe2b468124947))



<a name="v1.1.0"></a>
## v1.1.0 (2019-05-01)


#### Breaking Changes

*   DomoError was removed and replaced by PitchforkError ([296efb91](https://github.com/quantumZebraPDX/domo-pitchfork/commit/296efb9132a94e25732a21950ad2c0c8c1afee6e))

#### Features

*   new error that impl's std::error::Error ([296efb91](https://github.com/quantumZebraPDX/domo-pitchfork/commit/296efb9132a94e25732a21950ad2c0c8c1afee6e))



<a name="v1.0.1"></a>
### v1.0.1 (2019-04-25)


#### Bug Fixes

*   set http failure codes to return DomoError ([a5d16ce2](https://github.com/quantumZebraPDX/domo-pitchfork/commit/a5d16ce2cefcc5971cb6c7c3e65a937c25430f02))

#### Breaking Changes

*   set http failure codes to return DomoError ([a5d16ce2](https://github.com/quantumZebraPDX/domo-pitchfork/commit/a5d16ce2cefcc5971cb6c7c3e65a937c25430f02))



<a name="v1.0.0"></a>
## v1.0.0 (2019-04-16)


#### Documentation

*   add example to README ([18d7b1ba](https://github.com/quantumZebraPDX/domo-pitchfork/commit/18d7b1baf51da53d49700e2d7d7026a1b9bce71e))
*   remove empty links from changelog ([e69c3462](https://github.com/quantumZebraPDX/domo-pitchfork/commit/e69c3462e3716befd3128d534d9ca5d02e3021e3))
*   Add changelog ([91941cde](https://github.com/quantumZebraPDX/domo-pitchfork/commit/91941cded1265001645ef61fda5f38e1011a0d52))



<a name="v1.0.0-rc2"></a>
## v1.0.0-rc2 (2019-04-16)


#### Documentation

*   update package metadata ([482bdb14](https://github.com/quantumZebraPDX/domo-pitchfork/commit/482bdb14657927757476baa51136b5cfb09d29de))
*   Add more doc-tests ([18a6a96a](https://github.com/quantumZebraPDX/domo-pitchfork/commit/18a6a96acfabf253bf905768d6a0e1e0081ccf4d))



<a name="v1.0.0-rc"></a>
## v1.0.0-rc (2019-04-12)


#### Documentation

*   update package metadata ([482bdb14](https://github.com/quantumZebraPDX/domo-pitchfork/commit/482bdb14657927757476baa51136b5cfb09d29de))
*   Add more doc-tests ([18a6a96a](https://github.com/quantumZebraPDX/domo-pitchfork/commit/18a6a96acfabf253bf905768d6a0e1e0081ccf4d))

#### Refactor

* **pitchfork:**  refactor lib ([2436b0be](https://github.com/quantumZebraPDX/domo-pitchfork/commit/2436b0be297793bc74375efa8968668d842ac13a))
* **ripdomo:**  update to domo_pitchfork v1.0.0 ([a2e310bb](https://github.com/quantumZebraPDX/domo-pitchfork/commit/a2e310bbaa778435b156f60f4212bc101f5ddf43))

#### Breaking Changes

* **pitchfork:**
  *  change a param to correct type ([f953f962](https://github.com/quantumZebraPDX/domo-pitchfork/commit/f953f9627dab447291a5c7dd00f85c0127419ead))
  *  refactor lib ([2436b0be](https://github.com/quantumZebraPDX/domo-pitchfork/commit/2436b0be297793bc74375efa8968668d842ac13a))

#### Bug Fixes

* **pitchfork:**
  *  fix tests and streams util helper ([b63a6679](https://github.com/quantumZebraPDX/domo-pitchfork/commit/b63a66799e676aa27d7fc0508d5422918c54d560))
  *  change a param to correct type ([f953f962](https://github.com/quantumZebraPDX/domo-pitchfork/commit/f953f9627dab447291a5c7dd00f85c0127419ead))

#### Chore

* **pitchfork:**
  *  split lib and bin into 2 repos ([f0bd2c50](https://github.com/quantumZebraPDX/domo-pitchfork/commit/f0bd2c50e1cb8b36d3b06c3a2feaf640a91081d7))
  *  update deps ([5bc065d5](https://github.com/quantumZebraPDX/domo-pitchfork/commit/5bc065d50e627a7123e30bc66573c0aafce2fc84))
  *  renamed directory ([f96c6670](https://github.com/quantumZebraPDX/domo-pitchfork/commit/f96c6670c160e9a246563fdf3f482c11ed89226b))
