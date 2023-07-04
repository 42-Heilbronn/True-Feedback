const test = `[
    {
        "id": 3,
        "evaluation": {
            "team": "schibane's group-1",
            "project": "push_swap",
            "begin_at": "2023-06-21T21:30:00"
        }
    },
    {
        "id": 5,
        "evaluation": {
            "team": "schibane's group-1",
            "project": "push_swap",
            "begin_at": "2023-06-21T21:30:00"
        }
    }
]`;

const form_test = `
{
    "id": 5,
    "evaluation": {
        "team": "schibane's group-1",
        "project": "push_swap",
        "begin_at": "2023-06-21T21:30:00Z",
        "correcteds": [
            "schibane"
        ],
        "corrector": "oemelyan"
    },
    "fields": [
        {
            "key": "understanding",
            "name": "The code was thoroughly understood",
            "description": "Any Questions regarding the overall structure, design choices and individual functions could be answered flawlessly.",
            "data_type": {
                "Range": [
                    0,
                    10
                ]
            }
        },
        {
            "key": "uniqueness",
            "name": "The solution was unique",
            "description": "The solution provided a fresh perspective or approach that set it apart from conventional methods or existing alternatives?",
            "data_type": {
                "Range": [
                    0,
                    10
                ]
            }
        },
        {
            "key": "friendliness",
            "name": "The evaluation was very pleasant",
            "description": "The atmosphere throughout the entire process was very friendly. There was no discomfort and no uneasiness.",
            "data_type": {
                "Range": [
                    0,
                    10
                ]
            }
        },
        {
            "key": "comment",
            "name": "Comment",
            "description": "Optional comment you would like to share with bocal",
            "data_type": {
                "String": 1024
            }
        }
    ]
}`;

class EvalInfo
{
    constructor(peer)
    {
        this.peer = peer;
        this.eval_slot;
        this.questions;
        this.popup;
    }
}

const evals = new Map();
hasChanged = false;

// fetch('https://webhook.site/5e872038-ca12-410f-aec6-7bd62c9008ee')
// .then(res => res.json())
// .then(json => console.log(json));

create();
window.setInterval(create, 300000); //5 mins

function create()
{
    console.log("create");
    let missing = JSON.parse(test);
    missing.forEach(element => {
        if (evals.has(element.id) == false)
        {
            evals.set(element.id, new EvalInfo(element.evaluation));
            create_eval(element.id);
        }
    });
}

function create_eval(id)
{
    let eval_list = document.getElementById("collapseEvaluations");
    let eval = document.createElement("div");

    eval.classList.add("project-item", "reminder", "event");

    eval.innerHTML = `
    <div class="project-item-text"></div>
    <div class="project-item-actions"><a href="#">Give Feedback</a></div>`; //not just a, because that's also how intra42 does it. WHy do they do that? Dunno
    eval.firstElementChild.innerText = `Please submit honest feedback for your eval with ${evals.get(id).peer.team}'s ${evals.get(id).peer.project}`;
    eval.lastElementChild.firstElementChild.addEventListener("click", function() {showPopup(id)}); //adds a function call to show the popup

    eval_list.appendChild(eval);
    evals.get(id).eval_slot = eval;
    create_popup(id);
}

function create_popup(id) //rewrite because of possible xss injections
{
    let popup = document.createElement('div');
    let content = JSON.parse(form_test).fields;

    popup.style = "position: fixed; width: 100%; height: 100%; top: 0; left: 0; background: rgba(0,0,0,0.5); z-index: 9999; display: flex; justify-content: center; align-items: center; visibility: hidden;";
    popup.innerHTML = `
    <form style="background: #ffffff; padding: 20px; width: 400px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2); display: flex; flex-direction: column; gap: 20px; position: relative;">
        <span class="iconf-delete-2-1" style="position: absolute; top: 20px; right: 20px; color: red; cursor: pointer;"></span>
        <h1></h1>
        <div style="display: flex; flex-direction: column; gap: 10px;"></div>
        <span class="btn btn-primary" style="margin: 0 auto; border-radius: 5px; font-size: 17px; padding: 6px 18px;">Submit</span>
    </form>`;

    popup.firstElementChild.firstElementChild.nextElementSibling.innerText = `🔊 Feedback for ${evals.get(id).peer.team} 🔊`;

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

function showPopup(id)
{
    if (evals.get(id).popup.style.visibility == "hidden")
        evals.get(id).popup.style.visibility  = "visible";
    else
        evals.get(id).popup.style.visibility  = "hidden";
}

function submitForm(id)
{
    let data = {};

    if (hasChanged == false)
        return alert("Please note that your form submission appears to be incomplete as none of the sliders have been adjusted. To ensure accurate information, kindly review and adjust the sliders accordingly before resubmitting. Thank you for your cooperation.");
    
    Array.from(evals.get(id).popup.firstElementChild).forEach(element => {
        console.log(element);
        data[element.id] = element.value;
    });
    console.log(data);
    // fetch("https://reqbin.com/echo/post/json", {
    // method: "POST",
    // body: JSON.stringify({
    //     understanding: 5,
    //     uniqueness: 4,
    //     friendliness: 3,
    //     comment: "ey"
    // }),
    // headers: {
    //     "Content-type": "application/json; charset=UTF-8"
    // }
    // }).then(res => res.json()).then(json => console.log(json));
    evals.get(id).eval_slot.remove();
    evals.get(id).popup.remove();
    evals.delete(id);
}
