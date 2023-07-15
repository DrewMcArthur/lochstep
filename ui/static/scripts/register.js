const enablePasskeyAuth = async () =>
  true &&
  (await PublicKeyCredential.isUserVerifyingPlatformAuthenticatorAvailable()
    .then((passkeyAuthAvailable) => passkeyAuthAvailable)
    .catch((e) => false));

const showPasswordField = () => {
  htmx.removeClass(htmx.find("input.password"), "hidden");
};

const register = async () => {
  if (await enablePasskeyAuth()) {
    registerPasskey();
  } else {
    registerPassword();
  }
};

const registerPassword = async () => {
  const password = htmx.find("input.password").value;
  if (password == "") {
    showPasswordField();
    return;
  }
  const username = htmx.find("input.username").value;

  const res = await fetch("/auth/password/registration/create", {
    method: "POST",
    body: JSON.stringify({
      username,
      password,
    }),
  });

  console.log("POST /auth/password/registration/create response: " + res);
};

const registerPasskey = async () => {
  await getPasskeyRegistrationOptions()
    .then((opts) => {
      console.log("got passkey registration options: " + JSON.stringify(opts));
      return opts;
    })
    .then((opts) => generateCredentials(opts))
    .then((creds) => createPasskeyRegistration(creds))
    .catch((e) => console.error("error creating passkey registration: " + e));
};

const getPasskeyRegistrationOptions = async () => {
  return (
    await fetch("/auth/passkey/registration/options", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        username: htmx.find("input.username").value,
      }),
    })
  ).json();
};

const generateCredentials = async (opts) => {
  opts.publicKey.challenge = Base64.toUint8Array(opts.publicKey.challenge);
  opts.publicKey.user.id = Base64.toUint8Array(opts.publicKey.user.id);

  return await navigator.credentials.create({
    publicKey: opts.publicKey,
  });
};

// TODO: convert to typescript and strongly type credential here
const createPasskeyRegistration = async (credential) => {
  return await fetch("/auth/passkey/registration/create", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      id: credential.id,
      rawId: Base64.fromUint8Array(new Uint8Array(credential.rawId), true),
      type: credential.type,
      response: {
        attestationObject: Base64.fromUint8Array(
          new Uint8Array(credential.response.attestationObject),
          true
        ),
        clientDataJSON: Base64.fromUint8Array(
          new Uint8Array(credential.response.clientDataJSON),
          true
        ),
      },
    }),
  });
};

htmx.find("section.register button").addEventListener("click", register);
