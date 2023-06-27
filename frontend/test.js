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

// fetch('https://webhook.site/5e872038-ca12-410f-aec6-7bd62c9008ee')
// .then(res => res.json())
// .then(json => console.log(json));

create();

function create()
{
    let missing = JSON.parse(test);
    missing.forEach(element => {
        evals.set(element.id, new EvalInfo(element.evaluation));
        create_eval(element.id);
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

    popup.style = "position: fixed; width: 100%; height: 100%; top: 0; left: 0; background: rgba(0,0,0,0.5); z-index: 9999; display: flex; justify-content: center; align-items: center;";
    popup.style.visibility = "hidden";
    popup.innerHTML = `
    <form style="background: #ffffff; padding: 20px; width: 400px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2); display: flex; flex-direction: column; gap: 20px; position: relative;">
        <span class="iconf-delete-2-1" style="position: absolute; top: 20px; right: 20px; color: red; cursor: pointer;"></span>
        <h1>🔊 Feedback for ${evals.get(id).peer.team} 🔊</h1>
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
    popup.firstElementChild.firstElementChild.addEventListener("click", function() {showPopup(id)}); //adds a function call to hide the popup, needs to be function in a function bec js
    popup.firstElementChild.lastElementChild.addEventListener("click", function() {submitForm(id)});

    document.body.appendChild(popup);
    evals.get(id).popup = popup;
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
    console.log(evals.get(id).popup.firstElementChild[0]);
    // fetch("https://reqbin.com/echo/post/json", {
    // method: "POST",
    // body: JSON.stringify({
    //     understanding: 5,
    //     uniqueness: 4,
    //     friendliness: 3,
    //     comment: "gay"
    // }),
    // headers: {
    //     "Content-type": "application/json; charset=UTF-8"
    // }
    // }).then(res => res.json()).then(json => console.log(json));
    evals.get(id).eval_slot.remove();
    evals.get(id).popup.remove();
    evals.delete(id);
}
