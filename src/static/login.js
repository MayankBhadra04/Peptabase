
document.addEventListener("DOMContentLoaded", function () {
    const form = document.getElementById("my-form");

    // form.addEventListener("submit", function (event) {
        // event.preventDefault();
        // const formData = {
        //     username: document.getElementById("username").value,
        //     password: document.getElementById("password").value,
        // };

        // console.log(formData);

        // const xhttp = new XMLHttpRequest();
        // xhttp.open("POST", "https://aptabase.shuttleapp.rs/v1/auth/login");
        // xhttp.setRequestHeader("Content-Type", "application/json;charset=UTF-8");
        // xhttp.send(
        //     JSON.stringify({
        //         email: formData.username,
        //         password: formData.password,
        //     })
        // );
        //
        // console.log(xhttp.responseText);

        form.addEventListener('submit', function(event) {
            event.preventDefault(); // Prevent the default form submission behavior

            // Get form data
            const formData = {
                email: document.getElementById("username").value,
                password: document.getElementById("password").value,
            };

            // Make the API call
            fetch('https://aptabase.shuttleapp.rs/v1/auth/login', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(formData)
            })
                .then(response => {
                    console.log(response);
                    const token = response.headers.get('token');
                    console.log(token);
                    localStorage.setItem('authtoken', token);
                });
        });
});