class Device {
  constructor({ id, type, state = "connected", meta = {} }) {
    this.id = id;
    this.type = type;
    this.state = state;
    this.meta = meta;
  }
}

module.exports = Device;
