# H3-Rust

This crate contains bindings to [Uber's H3 Library]. It is currently a work in progress, the
list of the H3 C API that is covered by the crate is captured below:

- Indexing
  - [X] `geoToH3`
  - [X] `h3ToGeo`
  - [ ] `h3ToGeoBoundary`
- Inspection
  - [X] `h3GetResolution`
  - [X] `h3GetBaseCell`
  - [X] `stringToH3`
  - [ ] `h3ToString`
  - [ ] `h3IsResClassIII`
  - [ ] `h3IsPentagon`
- Traversal
  - [ ] `kRing`
  - [ ] `maxKringSize`
  - [ ] `kRingDistances`
  - [ ] `hexRange`
  - [ ] `hexRangeDistances`
  - [ ] `hexRanges`
  - [ ] `hexRing`
  - [ ] `h3Line`
  - [ ] `h3LineSize`
  - [ ] `h3Distance`
- Hierarchy
  - [ ] `h3ToParent`
  - [ ] `maxH3ToChildrenSize`
  - [ ] `compact`
  - [ ] `uncompact`
  - [ ] `maxUncompactSize`
- Regions
  - [ ] `polyfill`
  - [ ] `maxPolyfillSize`
  - [ ] `h3SetToLinkedGeo`
- Unidirectional Edges
  - [ ] `h3IndexesAreNeighbors`
  - [ ] `getH3UnidirectionalEdge`
  - [ ] `h3UnidirectionalEdgeIsValid`
  - [ ] `getOriginH3IndexFromUnidirectionalEdge`
  - [ ] `getDestinationH3IndexFromUnidirectionalEdge`
  - [ ] `getH3IndexesFromUnidirectionalEdge`
  - [ ] `getH3UnidirectionalEdgesFromHexagon`
  - [ ] `getH3UnidirectionalEdgeBoundary`
- Miscellaneous
  - [ ] `degsToRads`
  - [ ] `radsToDegs`
  - [ ] `hexAreaKm2`
  - [ ] `hexAreaM2`
  - [ ] `edgeLengthKm`
  - [ ] `edgeLengthM`
  - [ ] `numHexagons`
  - [ ] `getRes0Indexes`
  - [ ] `res0IndexCount`

[Uber's H3 Library]: https://uber.github.io/h3/#/
