import test from "ava";

import { getFiles, setFiles } from "../index.js";

test("getFiles", (t) => {
  const files = getFiles();
  t.true(Array.isArray(files), "getFiles should return an array");
});

test("setFiles", (t) => {
  t.true(setFiles(["C:\\Users\\606\\Downloads\\neovide.msi"]), "setFiles is error");
});
