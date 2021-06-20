import("../pkg").then(hd => {
    const wrapper = hd.SignupValueWrapper;

    // Initialize constraint system
    let cs = hd.signup();
    let component = "Signup";

    // A function that connects a HTML element to a constraint system variable
    function bind(variable) {
        let box = document.getElementById(variable);
        let help = document.getElementById(variable + "-help");
        // Send events to the constraint system
        box.addEventListener("input", () => {
            cs.set_variable(component, variable, wrapper.String(box.value));
            cs.update();
        })
        // Receive events from the constraint system
        cs.subscribe(component, variable,
            v => {
                box.classList.remove("is-invalid");
                box.classList.add("is-valid");
                box.value = v;
                help.textContent = "";
            },
            _ => { }, // Handle pending-events
            e => {
                box.classList.remove("is-valid");
                box.classList.add("is-invalid");
                help.textContent = e;
            }
        );
    }

    bind("username");
    bind("email");
    bind("password1");
    bind("password2");

    let passwordHelp = document.getElementById("password-help");
    cs.subscribe(component, "equal_passwords",
        v => {
            if (v) {
                passwordHelp.textContent = "Passwords must be equal"
            } else {
                passwordHelp.textContent = ""
            }
        }
    );

    // Bind register button
    let register = document.getElementById("register");
    let help = document.getElementById("register-help");
    let agree = document.getElementById("agree");
    agree.addEventListener("click", () => {
        cs.set_variable(component, "agreed", wrapper.bool(agree.checked));
        cs.update();
    });
    cs.subscribe(component, "button_enabled",
        v => {
            register.disabled = !v;
            if (v) {
                help.textContent = "";
            } else {
                help.textContent = "Disabled due to errors";
            }
        },
        _ => { },
        e => {
            register.disabled = true;
            help.textContent = "Disabled due to errors";
        }
    );
    cs.update();
})