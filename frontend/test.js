const evals = new Map();
const popups = new Map();

create();

function create()
{
    if (true) //check for api call if the user has any finished evals
    {
        create_eval("Patrick");
        create_form("Patrick");
        create_eval("Gerhard");
        create_form("Gerhard");
    }
}

function create_eval(name)
{
    let eval_list = document.getElementById("collapseEvaluations");
    let eval = document.createElement("div");

    eval.classList.add("project-item", "reminder", "event");
    eval.innerHTML = `
    <div class="project-item-text">Please submit honest feedback for your eval with ${name}</div>
    <div class="project-item-actions"><a href="#">Give Feedback</a></div>`; //not just a, because that's also how intra42 does it. WHy do they do that? Dunno
    eval.lastElementChild.firstElementChild.addEventListener("click", function() {showPopup(name)}); //adds a function call to show the popup to the a tag

    eval_list.appendChild(eval);
    evals.set(name, eval);
}

function create_form(name)
{
    let popup = document.createElement('div');

    popup.style = "position: fixed; width: 100%; height: 100%; top: 0; left: 0; background: rgba(0,0,0,0.5); z-index: 9999; display: flex; justify-content: center; align-items: center;";
    popup.style.visibility = "hidden";
    popup.innerHTML = `
    <form style="background: #ffffff; padding: 20px; width: 400px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2); display: flex; flex-direction: column; gap: 20px; position: relative;">
        <span class="iconf-delete-2-1" style="position: absolute; top: 20px; right: 20px; color: red; cursor: pointer;"></span>
        <h1>🔊 Feedback for ${name} 🔊</h1>
        <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.</p>
        <div style="display: flex; flex-direction: column; gap: 10px;">
            <label for="q1" style="margin-bottom: 0px;">Question 1</label>
            <p style="font-size: 13px; margin: 0px;">Description</p>
            <input type="range" id="q1" name="q1" min="0" max="5" value="0">
            <div>
                <p style="font-size: 13px; margin: 0px; color: red; float: left;">Disagree</p>
                <p style="font-size: 13px; margin: 0px; color: #00babc; float: right;">Agree</p>
            </div>
            <label for="q2" style="margin-bottom: 0px;">Question 2</label>
            <p style="font-size: 13px; margin: 0px;">Description</p>
            <input type="range" id="q1" name="q1" min="0" max="5" value="0">
            <div>
                <p style="font-size: 13px; margin: 0px; color: red; float: left;">Disagree</p>
                <p style="font-size: 13px; margin: 0px; color: #00babc; float: right;">Agree</p>
            </div>
            <label for="q3" style="margin-bottom: 0px;">Question 3</label>
            <p style="font-size: 13px; margin: 0px;">Description</p>
            <input type="range" id="q1" name="q1" min="0" max="5" value="0">
            <div>
                <p style="font-size: 13px; margin: 0px; color: red; float: left;">Disagree</p>
                <p style="font-size: 13px; margin: 0px; color: #00babc; float: right;">Agree</p>
            </div>
        </div>
        <div style="display: flex; flex-direction: column; gap: 10px;">
            <label for="feedback">(Optional) Provide additional feedback:</label>
            <textarea id="feedback" name="feedback" rows="4" style="resize:vertical;"></textarea>
        </div>
        <span class="btn btn-primary" style="margin: 0 auto; border-radius: 5px; font-size: 17px; padding: 6px 18px;">Submit</span>
    </form>`;
    popup.firstElementChild.firstElementChild.addEventListener("click", function() {showPopup(name)}); //adds a function call to hide the popup to the span tag, needs to be function in a function bec js
    console.log(popup.firstElementChild.lastElementChild);
    popup.firstElementChild.lastElementChild.addEventListener("click", function() {submitForm(name)});

    document.body.appendChild(popup);
    popups.set(name, popup);
}

function showPopup(name)
{
    if (popups.get(name).style.visibility == "hidden")
        popups.get(name).style.visibility  = "visible";
    else
        popups.get(name).style.visibility  = "hidden";
}

function submitForm(name)
{
    popups.get(name).remove();
    evals.get(name).remove();
    popups.delete(name);
    evals.delete(name);
}

//         // Get the form data
//         let data = new FormData(feedbackForm);

//         // Send the form data to the server
//         GM_xmlhttpRequest({
//             method: 'POST',
//             url: 'https://webhook.site/5e872038-ca12-410f-aec6-7bd62c9008ee',
//             data: data,
//             headers: {
//                 'Content-Type': 'application/x-www-form-urlencoded'
//             },
//             onload: function(response) {
//                 alert('Your feedback has been submitted!');
//                 popup.style.display = "none"; // close the popup
//             }
//         });
//     };
