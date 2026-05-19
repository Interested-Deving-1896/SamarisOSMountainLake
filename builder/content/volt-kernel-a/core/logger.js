function format(args) {
  return args
    .map((value) => {
      if (typeof value === "string") return value;
      try {
        return JSON.stringify(value);
      } catch {
        return String(value);
      }
    })
    .join(" ");
}

module.exports = {
  info(...args) {
    console.log(`[SAMARIS] ${format(args)}`);
  },
  warn(...args) {
    console.warn(`[SAMARIS] ${format(args)}`);
  },
  error(...args) {
    console.error(`[SAMARIS] ${format(args)}`);
  }
};
