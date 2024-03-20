
document.addEventListener("DOMContentLoaded", function () {
    const form = document.getElementById("my-form");

    form.addEventListener("submit", function (event) {
        event.preventDefault();
        const formData = {
            username: document.getElementById("username").value,
            password: document.getElementById("password").value,
        };

        console.log(formData);

        const xhttp = new XMLHttpRequest();
        xhttp.open("POST", "https://aptabase.shuttleapp.rs/v1/auth/login");
        xhttp.setRequestHeader("Content-Type", "application/json;charset=UTF-8");
        xhttp.send(
            JSON.stringify({
                email: formData.username,
                password: formData.password,
            })
        );

        console.log(xhttp.responseText);
    });
});