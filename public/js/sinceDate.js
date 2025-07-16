
function setDateText() {
    const UTCSpan = Date.parse(document.getElementById("date").innerText);
    const nowUTC = new Date()

    const diff = Math.abs(nowUTC - UTCSpan);
    const diffDays = (diff / 86400000).toFixed(2);
    /*
    Edits all the text so that javascript being disabled does not result in broken text
     */
    document.getElementById("since").innerText = `(${diffDays} days ago)`
}


setDateText();