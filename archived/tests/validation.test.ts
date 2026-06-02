import { describe, it, expect, afterAll, beforeAll } from "vitest";
import fs from "fs";
import path from "path";
import {
    validateInput,
    validatePath,
    validateImageFiles,
    validateSourceDir,
} from "../src/validation";

/**
 * * Creates temporary directories
 */

const TEST_DIR = path.join(process.cwd(), "test-fixtures");
const VALID_DIR = path.join(TEST_DIR, "valid");
const EMPTY_DIR = path.join(TEST_DIR, "empty");

beforeAll(() => {
    if (!fs.existsSync(TEST_DIR)) {
        fs.mkdirSync(TEST_DIR, { recursive: true });
    }
    if (!fs.existsSync(VALID_DIR)) {
        fs.mkdirSync(VALID_DIR);
    }
    if (!fs.existsSync(EMPTY_DIR)) {
        fs.mkdirSync(EMPTY_DIR);
    }

    fs.writeFileSync(path.join(VALID_DIR, "image1.jpg"), "fake jpg data");
    fs.writeFileSync(path.join(VALID_DIR, "image2.png"), "fake png data");
    fs.writeFileSync(path.join(VALID_DIR, "document.txt"), "not an image");
});

afterAll(() => {
    if (fs.existsSync(TEST_DIR)) {
        fs.rmSync(TEST_DIR, { recursive: true, force: true });
    }
});

describe("validateInput", () => {
    it("should reject empty input", () => {
        const result = validateInput("");
        expect(result.isValid).toBe(false);
        expect(result.error).toContain("cannot be empty");
    });

    it("should reject input with null bytes", () => {
        const result = validateInput("path\0hack");
        expect(result.isValid).toBe(false);
        expect(result.error).toContain("Invalid character/s in path");
    });

    it("should reject path traversal attempts", () => {
        const result = validateInput("../../etc/passwd");
        expect(result.isValid).toBe(false);
        expect(result.error).toContain("unsafe path");
    });

    it("should reject paths that are too long", () => {
        const longPath = "a".repeat(300);
        const result = validateInput(longPath);
        expect(result.isValid).toBe(false);
        expect(result.error).toContain("Path is too long");
    });

    it("should expect valid paths", () => {
        const result = validateInput("./images");
        expect(result.isValid).toBe(true);
        expect(result.error).toBeUndefined();
    });
    it("should trim white space", () => {
        const result = validateInput("  ./images");
        expect(result.isValid).toBe(true);
    });
});

describe("validatePath", () => {
    it("should reject non-existent paths", () => {
        const result = validatePath("/this/does/not/exist");
        expect(result.isValid).toBe(false);
        expect(result.error).toContain("Path does not exist.");
    });
    it("should reject files (not dirs)", () => {
        const filePath = path.join(VALID_DIR, "image1.jpg");
        const result = validatePath(filePath);
        expect(result.isValid).toBe(false);
        expect(result.error).toContain("not a directory");
    });
    it("should reject empty directories", () => {
        const result = validatePath(EMPTY_DIR);
        expect(result.isValid).toBe(false);
        expect(result.error).toContain("Empty Directory");
    });
    it("should accept valid directories with files", () => {
        const result = validatePath(VALID_DIR);
        expect(result.isValid).toBe(true);
    });
});

describe("validateImageFiles", () => {
    const EXTENSIONS = [".jpg", ".jpeg", ".png", ".avif", ".webp"];

    it("should reject directories with no supported images", () => {
        // Create temp dir with only .txt files
        const noImagesDir = path.join(TEST_DIR, "no-images");
        fs.mkdirSync(noImagesDir, { recursive: true });
        fs.writeFileSync(path.join(noImagesDir, "doc.txt"), "text");

        const result = validateImageFiles(noImagesDir, EXTENSIONS);

        expect(result.isValid).toBe(false);
        expect(result.error).toContain("No supported image files");

        // Cleanup
        fs.rmSync(noImagesDir, { recursive: true });
    });

    it("should accept directories with supported images", () => {
        const result = validateImageFiles(VALID_DIR, EXTENSIONS);
        expect(result.isValid).toBe(true);
        expect(result.error).toContain("2 image(s)");
    });

    it("should ignore non-image files", () => {
        const result = validateImageFiles(VALID_DIR, EXTENSIONS);
        // Has 2 images + 1 txt file, should only count images
        expect(result.error).toContain("2 image(s)");
    });

    it("should be case-insensitive for extensions", () => {
        const mixedCaseDir = path.join(TEST_DIR, "mixed-case");
        fs.mkdirSync(mixedCaseDir, { recursive: true });
        fs.writeFileSync(path.join(mixedCaseDir, "IMAGE.JPG"), "fake");
        fs.writeFileSync(path.join(mixedCaseDir, "photo.PNG"), "fake");

        const result = validateImageFiles(mixedCaseDir, EXTENSIONS);

        expect(result.isValid).toBe(true);
        expect(result.error).toContain("2 image(s)");

        // Cleanup
        fs.rmSync(mixedCaseDir, { recursive: true });
    });
});

describe("validateSourceDir (integration)", () => {
    const EXTENSIONS = [".jpg", ".jpeg", ".png", ".avif", ".webp"];

    it("should pass all validations for valid directory", () => {
        const result = validateSourceDir(VALID_DIR, EXTENSIONS);
        expect(result.isValid).toBe(true);
        expect(result.error).toContain("image(s)");
    });

    it("should fail on empty input", () => {
        const result = validateSourceDir("", EXTENSIONS);
        expect(result.isValid).toBe(false);
    });

    it("should fail on non-existent path", () => {
        const result = validateSourceDir("./does-not-exist", EXTENSIONS);
        expect(result.isValid).toBe(false);
    });

    it("should fail on directory with no images", () => {
        const result = validateSourceDir(EMPTY_DIR, EXTENSIONS);
        expect(result.isValid).toBe(false);
    });
});
