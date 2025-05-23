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

  function rollDice() {
    var playerName = document.querySelector(".playerName").value;
    var diceType = document.querySelector(".diceType").value;
    var bonus = parseInt(document.querySelector(".bonus").value);

    var obj = {
        name: playerName,
        dice_to_roll: {
            dice_used: "D" + diceType,
            bonus: bonus,
            result_rolled: 0
        },
        history: []
    };

    fetch("http://localhost:8080/roll", {
        method: "POST",
        headers: {
            "Content-type": "application/json"
        },
        body: JSON.stringify(obj)
    })
    .then(res => res.json())
    .then(data => {
        const historyList = document.getElementById("rollHistory");
        
        [...historyList.children].forEach(child => {
          if (child.textContent.trim() === "No rolls yet.") {
              historyList.removeChild(child);
          }
      });
       const rawResult = data.dice_to_roll.result_rolled - data.dice_to_roll.bonus;
        const newItem = document.createElement("li");
        newItem.textContent = `${data.name}: Rolled a ${data.dice_to_roll.result_rolled} with a '${data.dice_to_roll.dice_used}' (${rawResult}) Bonus: (${data.dice_to_roll.bonus})`;
        //newItem.textContent = `${data.name} rolled a ${data.dice_to_roll.dice_used}: ${data.dice_to_roll.result_rolled} (Bonus: ${data.dice_to_roll.bonus})`;
        historyList.appendChild(newItem);
    })
    .catch(err => {
        console.error("Fehler beim Verarbeiten der Antwort:", err);
    });
}




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





    
