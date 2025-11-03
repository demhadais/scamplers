import { SERVER_CONFIG } from "../config";

export const ENCRYPTION_ALGORITHM = {
  name: "AES-GCM",
  length: 256,
};

export class ApiKey {
  readonly encryptedKey: Uint8Array;
  readonly initializationVector: Uint8Array;
  readonly prefix: Uint8Array;
  readonly hash: string;
  private constructor({
    encryptedKey,
    initializationVector,
    prefix,
    hash,
  }: {
    encryptedKey: Uint8Array;
    initializationVector: Uint8Array;
    prefix: Uint8Array;
    hash: string;
  }) {
    this.encryptedKey = encryptedKey;
    this.initializationVector = initializationVector;
    this.prefix = prefix;
    this.hash = hash;
  }

  static async new(encryptionSecret: CryptoKey): Promise<ApiKey> {
    const apiKeyLength = 32;
    const apiKey = crypto.getRandomValues(new Uint8Array(apiKeyLength));

    const initializationVectorLength = 12; // 96 bits (https://developer.mozilla.org/en-US/docs/Web/API/AesGcmParams#iv)
    const initializationVector = crypto.getRandomValues(
      new Uint8Array(initializationVectorLength),
    );

    const encryptedApiKey = await crypto.subtle.encrypt(
      {
        iv: initializationVector,
        ...ENCRYPTION_ALGORITHM,
      },
      encryptionSecret,
      apiKey,
    );

    const prefix = apiKey.slice(
      0,
      SERVER_CONFIG.api_key_prefix_length! as number,
    );
    const hash = await Bun.password.hash(apiKey);

    return new ApiKey({
      encryptedKey: new Uint8Array(encryptedApiKey),
      initializationVector,
      prefix,
      hash,
    });
  }
}

async function encryptionSecret(filePath: string): Promise<CryptoKey> {
  const file = Bun.file(filePath);
  const usages: KeyUsage[] = ["decrypt", "encrypt"];

  const fileExists = await file.exists();

  // If the file has been created, import the key from there. If not, create one.
  const secret = fileExists
    ? await crypto.subtle.importKey(
        "raw",
        await file.arrayBuffer(),
        ENCRYPTION_ALGORITHM,
        false,
        usages,
      )
    : await crypto.subtle.generateKey(ENCRYPTION_ALGORITHM, true, usages);

  if (!fileExists) {
    Bun.write(file, await crypto.subtle.exportKey("raw", secret), {
      mode: 400,
    });
  }

  return secret;
}

export const API_KEY_ENCRYPTION_SECRET = await encryptionSecret(
  ".api-key-encryption-secret",
);
