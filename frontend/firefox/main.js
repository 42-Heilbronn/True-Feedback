class EvalInfo
{
    constructor(peer)
    {
        this.peer = peer; //peer object from get request
        this.eval_slot; //the eval slot element created
        this.popup; //the popup element created
    }
}

const SERVER_IP = "https://dev01.playground.extension.42heilbronn.de/api";
const evals = new Map(); //map that saves all EvalInfo objects
var hasChanged = false; //checks if the sliders have been adjusted at least once

//checks if the user has a cookie
//if not it redirects him to /login
browser.runtime.sendMessage("auth").then(res => {
    if (res != 200)
        window.location.href = `${SERVER_IP}/login`;
    create();
    window.setInterval(create, 300000); //5 mins
});

//creates an eval slot for every eval that hasn't got a feedback from the user yet
function create()
{
    browser.runtime.sendMessage("miss").then(res => {
        res.forEach(element => {
            if (evals.has(element.id) == false)
            {
                evals.set(element.id, new EvalInfo(element.evaluation));
                create_eval(element.id);
            }
        });
    });
}

function create_eval(id)
{
    let eval_list = document.getElementById("collapseEvaluations");
    let eval = document.createElement("div");

    eval.classList.add("project-item", "reminder", "event");

    eval.innerHTML = `
    <div class="project-item-text"></div>
    <div class="project-item-actions"><a href="#">Give Feedback</a></div>`; //not just a, because that's also how intra42 does it
    eval.firstElementChild.innerText = `Please submit honest feedback for your eval with ${evals.get(id).peer.team}'s ${evals.get(id).peer.project} by ${evals.get(id).peer.evaluation.corrector}`;
    eval.lastElementChild.firstElementChild.addEventListener("click", function() {showPopup(id)});

    eval_list.appendChild(eval);
    evals.get(id).eval_slot = eval;
}

//iterates through the details for the eval and creates as many elements as needed
function create_popup(id, content)
{
    let popup = document.createElement('div');

    popup.style = "position: fixed; width: 100%; height: 100%; top: 0; left: 0; background: rgba(0,0,0,0.5); z-index: 9999; display: flex; justify-content: center; align-items: center;";
    popup.innerHTML = `
    <form style="background: #ffffff; padding: 20px; width: 400px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2); display: flex; flex-direction: column; gap: 20px; position: relative;">
        <span class="iconf-delete-2-1" style="position: absolute; top: 20px; right: 20px; color: red; cursor: pointer;"></span>
        <h1></h1>
        <div style="display: flex; flex-direction: column; gap: 10px;"></div>
        <span class="btn btn-primary" style="margin: 0 auto; border-radius: 5px; font-size: 17px; padding: 6px 18px;">Submit</span>
    </form>`;

    popup.firstElementChild.firstElementChild.nextElementSibling.innerText = `🔊 Feedback for ${evals.get(id).peer.team} by ${evals.get(id).peer.evaluation.corrector}🔊`;

    content.forEach(element => {
        if (element.data_type.Range != null)
            create_slider(element, popup.firstElementChild.lastElementChild.previousElementSibling);
        else
            create_textarea(element, popup.firstElementChild.lastElementChild.previousElementSibling);
    });

    popup.firstElementChild.firstElementChild.addEventListener("click", function() {showPopup(id)}); //adds a function call to hide the popup, needs to be function in a function bec js
    popup.firstElementChild.lastElementChild.addEventListener("click", function() {submitForm(id)});

    document.body.appendChild(popup);
    evals.get(id).popup = popup;
}

function create_slider(content, content_div)
{
    let label = document.createElement("label");
    let description = document.createElement("p");
    let slider = document.createElement("input");
    let agree = document.createElement("div");

    label.htmlFor = content.key;
    label.style = "margin-bottom: 0px;";
    label.innerText = content.name;

    description.style = "font-size: 13px; margin:0px;";
    description.innerText = content.description;

    slider.type = "range";
    slider.id = content.key;
    slider.name = content.key;
    slider.min = content.data_type.Range[0];
    slider.max = content.data_type.Range[1];
    slider.value = content.data_type.Range[1] / 2;
    slider.onchange = function (){hasChanged = true;};

    agree.innerHTML = `
    <p style="font-size: 13px; margin: 0px; color: red; float: left;">Disagree</p>
    <p style="font-size: 13px; margin: 0px; color: #00babc; float: right;">Agree</p>`

    content_div.appendChild(label);
    content_div.appendChild(description);
    content_div.appendChild(slider);
    content_div.appendChild(agree);
}

function create_textarea(content, content_div)
{
    let label = document.createElement("label");
    let description = document.createElement("p");
    let textarea = document.createElement("textarea");

    label.htmlFor = content.key;
    label.style = "margin-bottom: 0px;";
    label.innerText = content.name;

    description.style = "font-size: 13px; margin:0px;";
    description.innerText = content.description;

    textarea.id = content.key;
    textarea.name = content.key;
    textarea.rows = 4;
    textarea.style = "resize: vertical;";

    content_div.appendChild(label);
    content_div.appendChild(description);
    content_div.appendChild(textarea);
}

//creates, hides or shows popup
function showPopup(id)
{
    if (evals.get(id).popup == undefined)
        browser.runtime.sendMessage({uri : `/feedback/${id}/info`}).then(res => create_popup(id, res.fields));
    else
    {
        if (evals.get(id).popup.style.visibility == "hidden")
            evals.get(id).popup.style.visibility  = "visible";
        else
            evals.get(id).popup.style.visibility  = "hidden";
    }
}

//extracts and POSTs data to the server
//slider values will be converted to integers
//deletes all the elements afterwards
function submitForm(id)
{
    let data = {};

    if (hasChanged == false)
        return alert("Please note that your form submission appears to be incomplete as none of the sliders have been adjusted. To ensure accurate information, kindly review and adjust the sliders accordingly before resubmitting. Thank you for your cooperation.");
    
    Array.from(evals.get(id).popup.firstElementChild).forEach(element => {  
        if (element.nodeName == "INPUT") //slider
            data[element.id] = parseInt(element.value);
        else //textbox
            data[element.id] = element.value;
    });

    browser.runtime.sendMessage({uri :`/feedback/${id}`, form: data}).then(function (res)
    {
        evals.get(id).eval_slot.remove();
        evals.get(id).popup.remove();
        evals.delete(id);
    });
}
