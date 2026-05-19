const { measureAsync, computeStats } = require("./benchmark");

class MatrixResult {
  constructor(name, dimensions, results) {
    this.name = name;
    this.dimensions = dimensions;
    this.results = results;
  }
}

async function runMatrix(name, dimensions, fn, { iterations = 50, warmup = 10 } = {}) {
  const results = [];

  function cartesianProduct(arrays) {
    if (arrays.length === 0) return [[]];
    const [first, ...rest] = arrays;
    const restProduct = cartesianProduct(rest);
    return first.flatMap((val) => restProduct.map((combo) => [val, ...combo]));
  }

  const keys = Object.keys(dimensions);
  const values = Object.values(dimensions);
  const combos = cartesianProduct(values);

  for (const combo of combos) {
    const params = {};
    for (let i = 0; i < keys.length; i++) params[keys[i]] = combo[i];

    for (let w = 0; w < warmup; w++) {
      try { await fn(params); } catch {}
    }

    const samples = [];
    for (let i = 0; i < iterations; i++) {
      const start = Number(process.hrtime.bigint());
      try { await fn(params); } catch {}
      samples.push(Number(process.hrtime.bigint()) - start);
    }

    results.push({ params, stats: computeStats(samples) });
  }

  return new MatrixResult(name, keys, results);
}

module.exports = { runMatrix, MatrixResult };
