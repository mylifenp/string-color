import * as mocha from "mocha";
import * as chai from "chai";
import { get_color } from "@mylifenp/string-color-wasm";

const expect = chai.expect;

const test_string = "A test string";
const hex_result = "#7cde5c";
const cmyk_result = "cmyk(44%, 0%, 58%, 12%)";
const hsl_result = "hsl(105, 66%, 61%)";
const hwb_result = "hwb(62, 36%, 12%)";

describe("string_color", () => {
  it("should return a hex color as default when return format not provided", () => {
    expect(get_color(test_string, "")).to.equal(hex_result);
  });
  it("should give another color when text is different", () => {
    expect(get_color(`${test_string},`, "")).to.not.equal(hex_result); // comma added
  });
  it("should return a hex color when return format is hex", () => {
    expect(get_color(test_string, "hex")).to.equal(hex_result);
  });
  it("should return a cmyk color when return format is cmyk", () => {
    expect(get_color(test_string, "cmyk")).to.equal(cmyk_result);
  });
  it("should return a different cmyk color when text changes", () => {
    expect(get_color(`${test_string}test`, "cmyk")).to.not.equal(cmyk_result);
  });
  it("should return a hsl color when return format is hsl", () => {
    expect(get_color(test_string, "hsl")).to.equal(hsl_result);
  });
  it("should return a different hsl color when text changes", () => {
    expect(get_color(`${test_string}testhsl`, "hsl")).to.not.equal(cmyk_result);
  });
  it("should return a hwb color when return format is hwb", () => {
    expect(get_color(test_string, "hwb")).to.equal(hwb_result);
  });
  it("should return a different hwb color when text changes", () => {
    expect(get_color(`${test_string}test`, "hwb")).to.not.equal(cmyk_result);
  });
});
