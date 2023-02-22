import * as mocha from "mocha";
import * as chai from "chai";
import { ColorFormat, string_color } from "../lib";

const expect = chai.expect;

const test_string = "A test string";
const hex_result = "#7cde5c";
const cmyk_result = "cmyk(44%, 0%, 58%, 12%)";
const hsl_result = "hsl(105, 66%, 61%)";
const hwb_result = "hwb(62, 36%, 12%)";

describe("string_color", () => {
  it("should return a hex color as default when return format not provided", () => {
    expect(string_color(test_string)).to.equal(hex_result);
  });
  it("should give another color when text is different", () => {
    expect(string_color(`${test_string},`)).to.not.equal(hex_result); // comma added
  });
  it("should return a hex color when return format is hex", () => {
    expect(string_color(test_string, ColorFormat.HEX)).to.equal(hex_result);
  });
  it("should return a cmyk color when return format is cmyk", () => {
    expect(string_color(test_string, ColorFormat.CMYK)).to.equal(cmyk_result);
  });
  it("should return a different cmyk color when text changes", () => {
    expect(string_color(`${test_string}test`, ColorFormat.CMYK)).to.not.equal(
      cmyk_result
    );
  });
  it("should return a hsl color when return format is hsl", () => {
    expect(string_color(test_string, ColorFormat.HSL)).to.equal(hsl_result);
  });
  it("should return a different hsl color when text changes", () => {
    expect(string_color(`${test_string}testhsl`, ColorFormat.HSL)).to.not.equal(
      cmyk_result
    );
  });
  it("should return a hwb color when return format is hwb", () => {
    expect(string_color(test_string, ColorFormat.HWB)).to.equal(hwb_result);
  });
  it("should return a different hwb color when text changes", () => {
    expect(string_color(`${test_string}test`, ColorFormat.HWB)).to.not.equal(
      cmyk_result
    );
  });
});
