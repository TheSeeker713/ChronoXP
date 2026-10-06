import { describe, it, expect } from "vitest";
import { formatDistanceToNow, wordCount } from "../utils/time";

describe("formatDistanceToNow", () => {
  it("returns 'just now' for very recent dates", () => {
    expect(formatDistanceToNow(new Date())).toBe("just now");
  });

  it("returns seconds for recent dates", () => {
    const d = new Date(Date.now() - 30_000);
    expect(formatDistanceToNow(d)).toBe("30s ago");
  });

  it("returns minutes for older dates", () => {
    const d = new Date(Date.now() - 120_000);
    expect(formatDistanceToNow(d)).toBe("2m ago");
  });

  it("returns hours for very old dates", () => {
    const d = new Date(Date.now() - 2 * 60 * 60 * 1000);
    expect(formatDistanceToNow(d)).toBe("2h ago");
  });

  it("returns days for dates more than a day old", () => {
    const d = new Date(Date.now() - 2 * 24 * 60 * 60 * 1000);
    expect(formatDistanceToNow(d)).toBe("2d ago");
  });
});

describe("wordCount", () => {
  it("returns 0 for empty string", () => {
    expect(wordCount("")).toBe(0);
  });

  it("returns 0 for whitespace-only string", () => {
    expect(wordCount("   ")).toBe(0);
  });

  it("counts single word", () => {
    expect(wordCount("hello")).toBe(1);
  });

  it("counts multiple words", () => {
    expect(wordCount("hello world foo bar")).toBe(4);
  });

  it("handles multiple spaces", () => {
    expect(wordCount("hello   world")).toBe(2);
  });
});
