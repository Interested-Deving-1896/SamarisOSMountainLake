const { describe, it } = require("node:test");
const assert = require("node:assert");

const VaultService = require("../services/vaultService");

describe("VaultService", () => {
  function mockUserService(secret) {
    return {
      getVaultIdentity() {
        return secret ? { username: "testuser", secret } : null;
      }
    };
  }

  it("encrypts and decrypts round-trips", async () => {
    const vault = new VaultService(null, mockUserService("my-password"));
    const plaintext = "Hello Samaris Vault!";
    const envelope = await vault.encryptForActiveUser(plaintext);

    assert.strictEqual(envelope.version, 1);
    assert.strictEqual(envelope.algorithm, "aes-256-gcm");
    assert.strictEqual(envelope.kdf, "scrypt");
    assert.ok(typeof envelope.salt === "string");
    assert.ok(typeof envelope.iv === "string");
    assert.ok(typeof envelope.data === "string");
    assert.ok(typeof envelope.authTag === "string");
    assert.notStrictEqual(envelope.data, plaintext);

    const decrypted = await vault.decryptForActiveUser(envelope);
    assert.strictEqual(decrypted, plaintext);
  });

  it("throws VAULT_LOCKED when no user is active", async () => {
    const vault = new VaultService(null, mockUserService(null));
    await assert.rejects(
      () => vault.encryptForActiveUser("test"),
      (err) => err.code === "VAULT_LOCKED"
    );
    await assert.rejects(
      () => vault.decryptForActiveUser({}),
      (err) => err.code === "VAULT_LOCKED"
    );
  });

  it("throws on wrong password decryption", async () => {
    const vault = new VaultService(null, mockUserService("password1"));
    const envelope = await vault.encryptForActiveUser("secret");

    const vault2 = new VaultService(null, mockUserService("password2"));
    await assert.rejects(
      () => vault2.decryptForActiveUser(envelope),
      (err) => /bad decrypt|unsupported state|unable to authenticate|vault/i.test(err.message)
    );
  });

  it("throws on tampered authTag", async () => {
    const vault = new VaultService(null, mockUserService("password"));
    const envelope = await vault.encryptForActiveUser("secret");
    envelope.authTag = "00".repeat(16);

    await assert.rejects(
      () => vault.decryptForActiveUser(envelope),
      (err) => /bad decrypt|unable to authenticate|vault/i.test(err.message)
    );
  });

  it("encryptString returns unique envelope each time", async () => {
    const vault = new VaultService(null, mockUserService("pass"));
    const env1 = await vault.encryptForActiveUser("same-text");
    const env2 = await vault.encryptForActiveUser("same-text");

    assert.notStrictEqual(env1.data, env2.data);
    assert.notStrictEqual(env1.iv, env2.iv);
    assert.notStrictEqual(env1.salt, env2.salt);
  });

  it("handles short plaintext", async () => {
    const vault = new VaultService(null, mockUserService("pass"));
    const plaintext = "A";
    const envelope = await vault.encryptForActiveUser(plaintext);
    assert.strictEqual(await vault.decryptForActiveUser(envelope), plaintext);
  });

  it("decryptForActiveUser returns empty string for null envelope", async () => {
    const vault = new VaultService(null, mockUserService("pass"));
    assert.strictEqual(await vault.decryptForActiveUser(null), "");
  });
});
