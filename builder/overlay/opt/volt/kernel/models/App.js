class App {
  constructor({ id, name, runtime = "app", permissions = [] }) {
    this.id = id;
    this.name = name;
    this.runtime = runtime;
    this.permissions = permissions;
  }
}

module.exports = App;
