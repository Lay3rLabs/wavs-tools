import { assert } from "chai";

describe("WAVS Tools Sanity Tests", () => {
  it("should pass basic arithmetic", () => {
    assert.equal(1 + 1, 2);
  });

  it("should pass string operations", () => {
    assert.equal("hello".toUpperCase(), "HELLO");
  });

  it("should pass array operations", () => {
    const arr = [1, 2, 3];
    assert.equal(arr.length, 3);
    assert.include(arr, 2);
  });

  // This test can be used to demonstrate skipped-test reporting
  it.skip("should demonstrate failure (skipped)", () => {
    assert.equal(2 + 2, 5, "This would fail: 2 + 2 should not equal 5");
  });

  // This test can be used to demonstrate failure reporting
  //   it("should demonstrate failure", () => {
  //     assert.equal(2 + 2, 5);
  //   });
});
