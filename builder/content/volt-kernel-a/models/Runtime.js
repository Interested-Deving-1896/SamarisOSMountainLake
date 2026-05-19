class Runtime {
  constructor({ id, kind, state = "running", target = null }) {
    this.id = id;
    this.kind = kind;
    this.state = state;
    this.target = target;
  }
}

module.exports = Runtime;
