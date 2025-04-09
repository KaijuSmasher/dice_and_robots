  const openModalButtons = document.querySelectorAll('[data-modal-target]')
  const closeModalButtons = document.querySelectorAll('[data-modal-close]')
  const overlay = document.getElementById('overlay')

  openModalButtons.forEach(button =>{
    button.addEventListener('click', () => {
        const modal = document.querySelector(button.dataset.modalTarget)
        openModal(modal)
    } )
  })

  closeModalButtons.forEach(button =>{
    button.addEventListener('click', () => {
        const modal = button.closest('.modal')
            closeModal(modal)
        })
    } )
  

  overlay.addEventListener('click', () =>{
    const modal = document.querySelectorAll(button.dataset.modalTarget)
    openModal(modal)
  })
  

  function openModal(modal){
    if (modal == null) return
    modal.classList.add('active')
    overlay.classList.add('active')
  }

  function closeModal(modal){
    if (modal == null) return
    modal.classList.remove('active')
    overlay.classList.remove('active')
  }


var playerName = document.querySelector(".playerName")
var diceType = document.querySelector(".diceType")
var bonus = document.querySelector(".bonus")
var button = document.querySelector(".buttonRoll")

button.addEventListener("click", () => {
    var obj ={
      name: playerName.value,
      dice_to_roll: {
          dice_used: "D" + diceType.value,
          bonus: parseInt(bonus.value),
          result_rolled: 0
      },
      history: []
    };

    fetch("http://localhost:8080/roll", {
        method:"POST",
        headers:{
            "Content-type":"application/json"
        },
        body:JSON.stringify(obj)
    });
})
.then(res => res.json())
.then(data => {
    const historyList = document.getElementById("rollHistory");
    const newItem = document.createElement("li");
    newItem.textContent = `Roll result: ${data.result_rolled}`;
    historyList.appendChild(newItem);
})
.catch(err => {
    console.error("Fehler beim Verarbeiten der Antwort:", err);
});

function clearRollHistory() {
  const historyList = document.getElementById("rollHistory");
  historyList.innerHTML = ""; // Alle bisherigen Einträge entfernen

  //Standard-Text wieder anzeigen
  const defaultItem = document.createElement("li");
  defaultItem.textContent = "No rolls yet.";
  historyList.appendChild(defaultItem);
}


function getModifier(score) {
  return Math.floor((score - 10) / 2);
}

function updateSkills() {
  const proficiency = parseInt(document.getElementById("proficiency").value) || 0;

  const stats = {
    str: getModifier(parseInt(document.getElementById("str").value) || 10),
    dex: getModifier(parseInt(document.getElementById("dex").value) || 10),
    wis: getModifier(parseInt(document.getElementById("wis").value) || 10),
    int: getModifier(parseInt(document.getElementById("int").value) || 10),
    cha: getModifier(parseInt(document.getElementById("cha").value) || 10)
  };

  const skillMap = {
    "acrobatics": "dex",
    "animalHandling": "wis",
    "arcana": "int",
    "athletics": "str",
    "deception": "cha",
    "history": "int",
    "insight": "wis",
    "intimidation": "cha",
    "investigation": "int",
    "medicine": "wis",
    "nature": "int",
    "perception": "wis",
    "performance": "cha",
    "persuasion": "cha",
    "religion": "int",
    "sleightOfHand": "dex",
    "stealth": "dex",
    "survival": "wis"
  };

  for (let skill in skillMap) {
      const mod = stats[skillMap[skill]];
      const isProficient = document.getElementById(`${skill}-check`).checked;
      const total = mod + (isProficient ? proficiency : 0);
      document.getElementById(`${skill}-score`).innerText = total;
  }
}

updateSkills();



    
