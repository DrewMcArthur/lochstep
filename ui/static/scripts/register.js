const enablePasswordlessAuth = true;
// if credentials isn't present, show the password input
PublicKeyCredential.isUserVerifyingPlatformAuthenticatorAvailable()
  .then((isAvailable) => {
    if (!(isAvailable && enablePasswordlessAuth)) {
      showPasswordField();
    }
  })
  .catch((e) => showPasswordField());

const showPasswordField = () => {
  document.getElementById("register-password").classList.remove("hidden");
};

// const beginRegistration = async () => {
//   const res = await fetch("/auth/begin-register", {
//     method: "POST",
//     body: JSON.stringify({
//       username: document.getElementById("username").value,
//       password: document.getElementById("register-password").value,
//     }),
//   });
//   console.log(res);
// };
