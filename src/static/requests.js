
function output(t){
    console.log(t);
    $(".output-span").text(t);
}

function set_permissions(e){
    e.preventDefault();
    submitter(e.target,(dt)=>{
        output("Permissions added: "+dt);
    });
    return false;
}

function list_rooms(e){
    e.preventDefault();
    submitter(e.target,(dt)=>{
        output("Room List"+dt);
    });
    return false;
}

function showNewUser(dt){
    output("Newuser : "+dt);
}

function showLogin(dt){
    console.log("Login Success : ",dt);
    $(".output-span").text("Login: " + dt);
}

function showNewGuest(dt){
    console.log("Created Guest : ",dt);
    $(".output-span").text("New Guest: " + dt);
}

function submitter(form,f){
    //form.preventDefault();
    f = f || showNewUser;
    console.log("form submitted : ",form);
    let form_ser = $(form).serializeArray();
    if (my_auth) form_ser.push({name:"auth",value:my_auth.k})
        else form_ser.push({name:"no-auth",value:"food"});
    console.log("Form Serialized = ",form_ser);
    
    jQuery.ajax({
        url : form.action,
        data: form_ser,
        type: form.method || "GET",
        success: (dt)=>{
            requests += 1;
            $(".request-count").text(requests);
            if (dt.err ) {
                    console.log("ERR,",dt.err);
                    $(".err-box").text("ERROR: " + dt.err);
            }
            if (dt.auth) {
                console.log("AUTH,",dt.auth);
                my_auth = dt.auth;
            }
            if (dt.data) f(dt.data);
        },

    });
    console.log("JQuery form happy");
    return false;
}

