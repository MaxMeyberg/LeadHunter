const contextInput = document.getElementById('context');
const outputArea = document.getElementById('output');
const generateBtn = document.getElementById('generate');
const copyBtn = document.getElementById('copy');
const sendBtn = document.getElementById('send');
const loadingDiv = document.getElementById('loading');
const emailDisplayAndActionsContainer = document.getElementById('output-area');

generateBtn.addEventListener('click', async () => {
  const context = contextInput.value.trim(); // This will be our user_prompt
  if (!context) {
    alert('Please add some context first!');
    return;
  }
  
  // reset UI
  outputArea.value = '';
  loadingDiv.classList.remove('hidden');
  emailDisplayAndActionsContainer.classList.add('hidden');
  generateBtn.disabled = true;
  
  chrome.tabs.query({ active: true, currentWindow: true }, tabs => {
    if (!tabs[0]?.id) {
      showError("No active LinkedIn tab found.");
      return;
    }
    
    chrome.tabs.sendMessage(
      tabs[0].id,
      { type: 'GENERATE_EMAIL', context }, // Send context from input
      response => {
        loadingDiv.classList.add('hidden');
        generateBtn.disabled = false;
        emailDisplayAndActionsContainer.classList.remove('hidden');
        
        if (chrome.runtime.lastError) {
          console.error("Error in popup after content script response:", chrome.runtime.lastError.message);
          outputArea.value = "Error: " + chrome.runtime.lastError.message + ". Check console.";
          return;
        }
        
        if (response && response.error) {
            outputArea.value = "Error: " + response.error;
        } else if (response && response.email) {
            outputArea.value = response.email;
        } else {
            outputArea.value = "Failed to generate email. No response or unexpected format from content script.";
        }
      }
    );
  });
});

copyBtn.addEventListener('click', () => {
  if (!outputArea.value) return;
  const original = copyBtn.innerHTML;
  navigator.clipboard.writeText(outputArea.value)
    .then(() => {
      copyBtn.innerHTML = `
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-green-500" viewBox="0 0 20 20" fill="currentColor">
          <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0
            01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0
            011.414 0z" clip-rule="evenodd"/>
        </svg>`;
      setTimeout(() => copyBtn.innerHTML = original, 1500);
    })
    .catch(err => console.error('Copy failed:', err));
});

sendBtn.addEventListener('click', () => {
  const body = encodeURIComponent(outputArea.value);
  const subject = encodeURIComponent('Quick Chat?');
  // opens default mail client
  window.location.href = `mailto:?subject=${subject}&body=${body}`;
});

function showError(msg) {
  loadingDiv.classList.add('hidden');
  generateBtn.disabled = false;
  outputArea.value = `Error: ${msg}`;
  emailDisplayAndActionsContainer.classList.remove('hidden');
}
