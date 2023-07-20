///
// for use with webauthn.io / passkey authentication.  disabled for now
///

const showPasswordField = () => {
  htmx.removeClass(htmx.find("input.password"), "hidden");
};

const enablePasskeyAuth = async () => {
  const enabled =
    false &&
    PublicKeyCredential &&
    (await PublicKeyCredential.isUserVerifyingPlatformAuthenticatorAvailable()
      .then((passkeyAuthAvailable) => passkeyAuthAvailable)
      .catch((e) => false));
  if (!enabled) showPasswordField();
  return enabled;
};

const isPasskeyAuthEnabled = enablePasskeyAuth();

const validate = () => {
  hideValidationError();
  let valid =
    htmx.find("input.password").value !== "" &&
    htmx.find("input.username").value !== "";
  if (!valid) showValidationError();
  return valid;
};

const showValidationError = () =>
  htmx.find(".validation-error").classList.remove("hidden");
const hideValidationError = () =>
  htmx.find(".validation-error").classList.add("hidden");

const hide = (className) => htmx.find(className).classList.add("hidden");
const show = (className) => htmx.find(className).classList.remove("hidden");

const register = async () => {
  const passKeyAuthEnabled = enablePasskeyAuth();
  if (!validate()) return;
  if (await passKeyAuthEnabled) registerPasskey();
  else registerPassword();
};

const registerPassword = async () => {
  const password = htmx.find("input.password").value;
  const username = htmx.find("input.username").value;

  const res = await fetch("/auth/password/registration/create", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      username,
      password,
    }),
  });

  if (res.ok) {
    hide(".register");
    show(".login");
  }

  console.log(
    "POST /auth/password/registration/create response: " + JSON.stringify(res)
  );
};

const registerPasskey = async () => {
  await getPasskeyRegistrationOptions()
    .then((res) => res.json())
    .then((opts) => generateCredentials(opts))
    .then((creds) => createPasskeyRegistration(creds))
    .catch((e) => console.error("error creating passkey registration: " + e));
};

const getPasskeyRegistrationOptions = async () => {
  return await fetch("/auth/passkey/registration/options", {
    method: "POST",
    credentials: "same-origin",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      username: htmx.find("input.username").value,
    }),
  });
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
    credentials: "same-origin",
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

const login = () => {};

htmx
  .find("section.register button.register")
  .addEventListener("click", register);

htmx.find("section.register button.login").addEventListener("click", login);
