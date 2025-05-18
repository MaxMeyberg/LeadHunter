// --- In-Page Modal and Button Injection for LinkedIn --- 

function createEmailTailorModal() {
    const modalId = 'emailTailorInPageModal';
    const existingModal = document.getElementById(modalId);
    if (existingModal) {
        existingModal.style.display = 'flex';
        // Ensure the overlay becomes visible if re-opening
        setTimeout(() => existingModal.classList.add('show'), 10); 
        return; // Don't recreate if it exists, just show it
    }

    // Create the quantum glass modal
    const modalOverlay = document.createElement('div');
    modalOverlay.id = modalId;
    modalOverlay.className = 'quantum-modal-overlay';
    
    const modalContent = document.createElement('div');
    modalContent.className = 'quantum-modal-content';
    
    // Add the HTML structure with modern elements
    modalContent.innerHTML = `
        <div class="modal-header">
            <div class="title-container">
                <div class="holographic-badge">
                    <svg class="mail-icon" viewBox="0 0 24 24">
                        <path d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/>
                    </svg>
                </div>
                <h2>LinkedIn <span class="gradient-text">AI Tailor</span></h2>
            </div>
            <button id="emailTailorInPageCloseTop" class="quantum-close-btn">
                <div class="close-x"></div>
            </button>
        </div>
        
        <div class="input-section">
            <label class="floating-label">
                <textarea id="emailTailorInPageContext" placeholder="Email Prompt" rows="3" placeholder=" "></textarea>
                <div class="active-border"></div>
            </label>
            <div class="hint-text">Mention their recent article, mutual connection, or why you're reaching out</div>
        </div>
        
        <button id="emailTailorInPageGenerate" class="quantum-generate-btn">
            <span class="btn-content">
                <span class="btn-icon">
                    <svg viewBox="0 0 24 24">
                        <path d="M13 2.05v2.02c3.95.49 7 3.85 7 7.93 0 3.21-1.92 6-4.72 7.28l-1.43-2.6c1.9-1.05 3.15-3.06 3.15-5.33 0-3.31-2.69-6-6-6H11V7l-4 3 4 3v-2h1c1.1 0 2 .9 2 2s-.9 2-2 2H9v2h2c2.21 0 4-1.79 4-4s-1.79-4-4-4H5V4h6c5.03 0 9.12 4.04 9 9.03-.13 4.49-3.87 8.13-8.37 8.37C6.04 21.12 2 17.03 2 12c0-5.22 4.22-9.5 9.5-9.5H13z"/>
                    </svg>
                </span>
                <span class="btn-text">Generate Email</span>
            </span>
            <div class="btn-shine"></div>
        </button>
        
        <div id="emailTailorInPageLoading" class="quantum-loading">
            <div class="particle-network"></div>
            <div class="loading-text">Composing your masterpiece...</div>
        </div>
        
        <div class="output-section">
            <label class="floating-label">
                <textarea id="emailTailorInPageOutput" placeholder="Tailored Email Output" rows="8" readonly></textarea>
                <div class="active-border"></div>
            </label>
            <div class="ai-signature">Crafted by AI</div>
        </div>
        
        <div class="modal-footer">
            <button id="emailTailorInPageCopy" class="quantum-action-btn copy-btn">
                <span class="btn-content">
                    <span class="btn-icon">
                        <svg viewBox="0 0 24 24">
                            <path d="M19 3h-4.18C14.4 1.84 13.3 1 12 1s-2.4.84-2.82 2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 0c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm6 16H6V5h2v2h8V5h2v14z"/>
                        </svg>
                    </span>
                    <span class="btn-text">Copy</span>
                </span>
            </button>
            <button id="emailTailorInPageSend" class="quantum-action-btn send-btn">
                <span class="btn-content">
                    <span class="btn-icon">
                        <svg viewBox="0 0 24 24">
                            <path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"/>
                        </svg>
                    </span>
                    <span class="btn-text">Send</span>
                </span>
            </button>
            <button id="emailTailorInPageCloseBottom" class="quantum-action-btn close-btn">
                <span class="btn-content">
                    <span class="btn-text">Close</span>
                </span>
            </button>
        </div>
    `;
    
    modalOverlay.appendChild(modalContent);
    document.body.appendChild(modalOverlay);
    
    // Show modal with transition
    setTimeout(() => modalOverlay.classList.add('show'), 10);

    // Get references to modal elements
    const contextInputInModal = document.getElementById('emailTailorInPageContext');
    const generateBtnInModal = document.getElementById('emailTailorInPageGenerate');
    const outputAreaInModal = document.getElementById('emailTailorInPageOutput');
    const loadingDivInModal = document.getElementById('emailTailorInPageLoading');
    const copyBtnInModal = document.getElementById('emailTailorInPageCopy');
    const sendBtnInModal = document.getElementById('emailTailorInPageSend');
    const closeBtnTop = document.getElementById('emailTailorInPageCloseTop');
    const closeBtnBottom = document.getElementById('emailTailorInPageCloseBottom');

    const hideModal = () => {
        modalOverlay.classList.remove('show');
        // Remove the modal from DOM after the transition completes
        setTimeout(() => {
            if (modalOverlay && modalOverlay.parentNode) {
                modalOverlay.parentNode.removeChild(modalOverlay);
            }
            // Clean up event listeners
            closeBtnTop.removeEventListener('click', hideModal);
            closeBtnBottom.removeEventListener('click', hideModal);
        }, 300); // Match this with your CSS transition duration
    };

    // Add event listeners for closing the modal
    closeBtnTop.addEventListener('click', hideModal);
    closeBtnBottom.addEventListener('click', hideModal);
    
    // Close when clicking outside the modal content
    modalOverlay.addEventListener('click', (e) => {
        if (e.target === modalOverlay) {
            hideModal();
        }
    });
    
    // Close with Escape key
    document.addEventListener('keydown', (e) => {
        if (e.key === 'Escape') {
            hideModal();
        }
    });

    // Event listener for the modal's Generate button
    generateBtnInModal.addEventListener('click', async () => {
        const user_prompt = contextInputInModal.value.trim();
        if (!user_prompt) {
            alert('Please enter a prompt or context for the email.');
            return;
        }

        const linkedin_url = window.location.href;

        // Show loading animation and disable button
        loadingDivInModal.style.display = 'flex'; // Assuming flex is used for centering
        outputAreaInModal.value = ''; // Clear previous output
        generateBtnInModal.disabled = true;
        generateBtnInModal.classList.add('quantum-generate-btn--loading');

        try {
            const apiResponse = await fetch('http://localhost:3000/generate-email', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ linkedin_url, user_prompt }),
            });

            if (!apiResponse.ok) {
                let errorMessage = `API Error: ${apiResponse.status}`;
                try {
                    const errorData = await apiResponse.json();
                    errorMessage = errorData.message || errorData.error || `API Error ${apiResponse.status}: ${apiResponse.statusText}`;
                } catch (e) {
                    errorMessage = `API Error ${apiResponse.status}: ${apiResponse.statusText || 'Failed to fetch'}`;
                }
                outputAreaInModal.value = `Error: ${errorMessage}`;
            } else {
                const data = await apiResponse.json();
                if (data.generated_email) {
                    // If we have an email address, prepend the "To" field
                    let emailContent = data.generated_email;
                    if (data.email_address) {
                        emailContent = `To: ${data.email_address}\n\n${emailContent}`;
                    }
                    outputAreaInModal.value = emailContent;
                } else if (data.error) {
                    outputAreaInModal.value = `Error: ${data.error}`;
                } else {
                    outputAreaInModal.value = "Error: Received no email content or error from the backend.";
                }
            }
        } catch (error) {
            console.error('In-Page Modal - Fetch API call failed:', error);
            outputAreaInModal.value = `Network error: ${error.message}. Ensure backend is running.`;
        } finally {
            // Hide loading animation and re-enable button
            loadingDivInModal.style.display = 'none';
            generateBtnInModal.disabled = false;
            generateBtnInModal.classList.remove('quantum-generate-btn--loading');
        }
    });

    // Event listener for the modal's Copy button
    copyBtnInModal.addEventListener('click', () => {
        if (!outputAreaInModal.value) return;
        const originalText = copyBtnInModal.querySelector('.btn-text').textContent;
        navigator.clipboard.writeText(outputAreaInModal.value)
            .then(() => {
                copyBtnInModal.querySelector('.btn-text').textContent = 'Copied!';
                copyBtnInModal.classList.add('success'); // Optional: for styling
                setTimeout(() => {
                    copyBtnInModal.querySelector('.btn-text').textContent = originalText;
                    copyBtnInModal.classList.remove('success');
                }, 1500);
            })
            .catch(err => {
                console.error('In-Page Modal - Copy failed:', err);
                alert('Failed to copy. Please try again or copy manually.');
            });
    });

    // Event listener for the modal's Send button
    sendBtnInModal.addEventListener('click', () => {
        const body = encodeURIComponent(outputAreaInModal.value);
        if (!body) {
            alert('No email content to send.');
            return;
        }
        // Simple subject, could be made configurable or smarter
        const subject = encodeURIComponent('Following up from LinkedIn'); 
        window.location.href = `mailto:?subject=${subject}&body=${body}`;
    });

    // Add the quantum CSS
    const style = document.createElement('style');
    style.textContent = `
        .quantum-modal-overlay {
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            background: rgba(0, 0, 0, 0.7);
            backdrop-filter: blur(8px);
            display: flex;
            justify-content: center;
            align-items: center;
            z-index: 20000;
            opacity: 0;
            transition: opacity 0.4s cubic-bezier(0.16, 1, 0.3, 1);
        }
        .quantum-modal-overlay.show {
            opacity: 1;
        }
        .quantum-modal-content {
            position: relative;
            width: 560px;
            max-width: 90vw;
            max-height: 95vh;
            overflow-y: auto;
            background: rgba(255, 255, 255, 0.85);
            backdrop-filter: blur(12px);
            border-radius: 24px;
            padding: 32px;
            box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25), 
                        inset 0 1px 0 rgba(255, 255, 255, 0.2);
            border: 1px solid rgba(255, 255, 255, 0.3);
            transform: translateY(20px) scale(0.98);
            opacity: 0;
            transition: all 0.4s cubic-bezier(0.16, 1, 0.3, 1);
            overflow: hidden;
        }
        .quantum-modal-overlay.show .quantum-modal-content {
            transform: translateY(0) scale(1);
            opacity: 1;
        }
        .modal-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 28px;
            padding-bottom: 16px;
            border-bottom: 1px solid rgba(0, 0, 0, 0.08);
        }
        .title-container {
            display: flex;
            align-items: center;
            gap: 16px;
        }
        .holographic-badge {
            width: 40px;
            height: 40px;
            border-radius: 12px;
            background: linear-gradient(135deg, #0073b1, #00b4ff);
            display: flex;
            align-items: center;
            justify-content: center;
            box-shadow: 0 4px 12px rgba(0, 115, 177, 0.3);
            position: relative;
            overflow: hidden;
        }
        .holographic-badge::before {
            content: '';
            position: absolute;
            top: -50%;
            left: -50%;
            width: 200%;
            height: 200%;
            background: linear-gradient(
                to bottom right,
                rgba(255, 255, 255, 0) 0%,
                rgba(255, 255, 255, 0) 30%,
                rgba(255, 255, 255, 0.3) 45%,
                rgba(255, 255, 255, 0) 60%,
                rgba(255, 255, 255, 0) 100%
            );
            transform: rotate(30deg);
            animation: holographic-shine 3s infinite;
        }
        .mail-icon {
            width: 22px;
            height: 22px;
            stroke: white;
            stroke-width: 1.5;
            fill: none;
            z-index: 1;
        }
        .modal-header h2 {
            font-size: 24px;
            font-weight: 700;
            color: #2c3e50;
            margin: 0;
        }
        .gradient-text {
            background: linear-gradient(90deg, #0073b1, #00b4ff);
            -webkit-background-clip: text;
            background-clip: text;
            color: transparent;
        }
        .quantum-close-btn {
            width: 36px;
            height: 36px;
            border-radius: 50%;
            background: rgba(0, 0, 0, 0.05);
            border: none;
            display: flex;
            align-items: center;
            justify-content: center;
            cursor: pointer;
            transition: all 0.2s ease;
            position: relative;
        }
        .quantum-close-btn:hover {
            background: rgba(0, 0, 0, 0.1);
        }
        .close-x {
            position: relative;
            width: 16px;
            height: 16px;
        }
        .close-x::before, .close-x::after {
            content: '';
            position: absolute;
            top: 50%;
            left: 50%;
            width: 100%;
            height: 2px;
            background: #95a5a6;
            transform-origin: center;
        }
        .close-x::before {
            transform: translate(-50%, -50%) rotate(45deg);
        }
        .close-x::after {
            transform: translate(-50%, -50%) rotate(-45deg);
        }
        .input-section, .output-section {
            margin-bottom: 24px;
            position: relative;
        }
        .floating-label {
            position: relative;
            display: block;
        }
        .floating-label span {
            position: absolute;
            top: 16px;
            left: 16px;
            font-size: 14px;
            color: #7f8c8d;
            pointer-events: none;
            transition: all 0.2s ease;
            transform-origin: left top;
        }
        .floating-label textarea {
            width: 100%;
            padding: 24px 16px 12px;
            border: 1px solid rgba(0, 0, 0, 0.1);
            border-radius: 12px;
            font-size: 14px;
            line-height: 1.5;
            resize: vertical;
            background: rgba(255, 255, 255, 0.7);
            transition: all 0.2s ease;
            box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
        }
        .floating-label textarea:focus {
            outline: none;
            border-color: #0073b1;
            box-shadow: 0 0 0 2px rgba(0, 115, 177, 0.2);
        }
        .floating-label textarea:focus + span,
        .floating-label textarea:not(:placeholder-shown) + span {
            transform: translateY(-10px) scale(0.85);
            color: #0073b1;
        }
        .active-border {
            position: absolute;
            bottom: 0;
            left: 0;
            width: 0;
            height: 2px;
            background: linear-gradient(90deg, #0073b1, #00b4ff);
            transition: width 0.3s ease;
        }
        .floating-label textarea:focus ~ .active-border {
            width: 100%;
        }
        .hint-text {
            font-size: 12px;
            color: #95a5a6;
            margin-top: 8px;
            padding-left: 4px;
        }
        .quantum-generate-btn {
            position: relative;
            width: 100%;
            padding: 16px;
            border: none;
            border-radius: 12px;
            background: linear-gradient(90deg, #0073b1, #00b4ff);
            color: white;
            font-weight: 600;
            font-size: 16px;
            cursor: pointer;
            overflow: hidden;
            transition: all 0.3s ease;
            margin-bottom: 24px;
            box-shadow: 0 4px 12px rgba(0, 115, 177, 0.3);
        }
        .quantum-generate-btn:hover {
            transform: translateY(-2px);
            box-shadow: 0 6px 16px rgba(0, 115, 177, 0.4);
        }
        .quantum-generate-btn:active {
            transform: translateY(0);
        }
        .btn-content {
            position: relative;
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 8px;
            z-index: 2;
        }
        .btn-icon {
            display: flex;
        }
        .btn-icon svg {
            width: 20px;
            height: 20px;
            fill: white;
        }
        .btn-shine {
            position: absolute;
            top: 0;
            left: -100%;
            width: 100%;
            height: 100%;
            background: linear-gradient(
                to right,
                rgba(255, 255, 255, 0) 0%,
                rgba(255, 255, 255, 0.3) 50%,
                rgba(255, 255, 255, 0) 100%
            );
            transition: all 0.6s ease;
        }
        .quantum-generate-btn:hover .btn-shine {
            left: 100%;
        }
        .quantum-loading {
            position: relative;
            height: 80px;
            border-radius: 12px;
            background: rgba(0, 0, 0, 0.03);
            margin-bottom: 24px;
            display: none;
            align-items: center;
            justify-content: center;
            overflow: hidden;
        }
        .particle-network {
            position: absolute;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            opacity: 0.3;
        }
        .loading-text {
            position: relative;
            font-size: 14px;
            color: #2c3e50;
            z-index: 2;
            display: flex;
            align-items: center;
            gap: 8px;
        }
        .loading-text::before {
            content: '';
            width: 20px;
            height: 20px;
            border: 3px solid rgba(0, 115, 177, 0.2);
            border-top-color: #0073b1;
            border-radius: 50%;
            animation: spin 1s linear infinite;
        }
        .ai-signature {
            font-size: 12px;
            color: #95a5a6;
            text-align: right;
            margin-top: 8px;
            font-style: italic;
        }
        .modal-footer {
            display: flex;
            justify-content: flex-end;
            gap: 12px;
            padding-top: 16px;
            border-top: 1px solid rgba(0, 0, 0, 0.08);
        }
        .quantum-action-btn {
            position: relative;
            padding: 12px 20px;
            border: none;
            border-radius: 8px;
            font-weight: 500;
            font-size: 14px;
            cursor: pointer;
            transition: all 0.2s ease;
            display: flex;
            align-items: center;
            gap: 8px;
            overflow: hidden;
        }
        .copy-btn {
            background: rgba(39, 174, 96, 0.4);
            color: #27ae60;
        }
        .copy-btn:hover {
            background: rgba(39, 174, 96, 0.5);
        }
        .send-btn {
            background: rgba(231, 67, 147, 0.4);
            color: #e84393;
        }
        .send-btn:hover {
            background: rgba(231, 67, 147, 0.5);
        }
        .close-btn {
            background: rgba(149, 165, 166, 0.4);
            color: #95a5a6;
        }
        .close-btn:hover {
            background: rgba(149, 165, 166, 0.5);
        }
        @keyframes holographic-shine {
            0% { transform: rotate(30deg) translate(-30%, -30%); }
            100% { transform: rotate(30deg) translate(30%, 30%); }
        }
        @keyframes spin {
            0% { transform: rotate(0deg); }
            100% { transform: rotate(360deg); }
        }
    `;
    document.head.appendChild(style);
}

function injectFloatingMailButton() {
    const fabId = 'emailTailorFloatingBtn';
    if (document.getElementById(fabId)) return;

    const fabButton = document.createElement('button');
    fabButton.id = fabId;
    fabButton.title = 'Tailor LinkedIn Email';
    fabButton.innerHTML = `
        <div class="holographic-orb">
            <div class="orb-core"></div>
            <svg class="orb-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <path stroke-linecap="round" stroke-linejoin="round" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
            </svg>
        </div>
    `;

    fabButton.style.cssText = `
        position: fixed;
        bottom: 30px;
        left: 30px;
        width: 68px;
        height: 68px;
        background: transparent;
        border: none;
        cursor: pointer;
        z-index: 19999;
        filter: drop-shadow(0 5px 15px rgba(0, 115, 177, 0.4));
        transition: transform 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
    `;

    // Add dynamic hover effects
    fabButton.onmouseenter = () => {
        fabButton.style.transform = 'scale(1.15)';
        document.querySelector(`#${fabId} .holographic-orb`).classList.add('orb-hover');
    };
    fabButton.onmouseleave = () => {
        fabButton.style.transform = 'scale(1)';
        document.querySelector(`#${fabId} .holographic-orb`).classList.remove('orb-hover');
    };
    fabButton.onmousedown = () => fabButton.style.transform = 'scale(0.95)';
    fabButton.onmouseup = () => fabButton.style.transform = 'scale(1.15)';

    // Inject CSS for the holographic effect
    const style = document.createElement('style');
    style.textContent = `
        .holographic-orb {
            position: relative;
            width: 100%;
            height: 100%;
            display: flex;
            align-items: center;
            justify-content: center;
            perspective: 1000px;
        }
        .orb-core {
            position: absolute;
            width: 100%;
            height: 100%;
            background: radial-gradient(circle at 30% 30%, 
                rgba(0, 180, 255, 0.8) 0%, 
                rgba(0, 115, 177, 0.9) 50%, 
                rgba(0, 80, 150, 1) 100%);
            border-radius: 50%;
            box-shadow: inset 0 0 20px rgba(255, 255, 255, 0.3);
            animation: orb-pulse 4s infinite alternate;
        }
        .orb-rings {
            position: absolute;
            width: 100%;
            height: 100%;
            transform-style: preserve-3d;
        }
        .ring {
            position: absolute;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            border: 2px solid rgba(255, 255, 255, 0.15);
            border-radius: 50%;
            transform-style: preserve-3d;
        }
        .ring-1 {
            transform: rotateX(70deg) rotateZ(0deg);
            animation: ring-rotate-1 12s linear infinite;
        }
        .ring-2 {
            transform: rotateX(70deg) rotateZ(120deg);
            animation: ring-rotate-2 15s linear infinite;
        }
        .ring-3 {
            transform: rotateX(70deg) rotateZ(240deg);
            animation: ring-rotate-3 18s linear infinite;
        }
        .orb-icon {
            position: relative;
            width: 32px;
            height: 32px;
            color: white;
            stroke-width: 1.5;
            z-index: 2;
            filter: drop-shadow(0 0 5px rgba(255, 255, 255, 0.7));
        }
        .orb-hover .orb-core {
            animation: orb-pulse-hover 2s infinite alternate;
        }
        .orb-hover .ring {
            border-color: rgba(255, 255, 255, 0.3);
        }
        @keyframes orb-pulse {
            0% { transform: scale(1); opacity: 0.9; }
            100% { transform: scale(1.05); opacity: 1; }
        }
        @keyframes orb-pulse-hover {
            0% { transform: scale(1); opacity: 0.9; }
            100% { transform: scale(1.1); opacity: 1; }
        }
        @keyframes ring-rotate-1 {
            0% { transform: rotateX(70deg) rotateZ(0deg); }
            100% { transform: rotateX(70deg) rotateZ(360deg); }
        }
        @keyframes ring-rotate-2 {
            0% { transform: rotateX(70deg) rotateZ(120deg); }
            100% { transform: rotateX(70deg) rotateZ(480deg); }
        }
        @keyframes ring-rotate-3 {
            0% { transform: rotateX(70deg) rotateZ(240deg); }
            100% { transform: rotateX(70deg) rotateZ(600deg); }
        }
    `;
    document.head.appendChild(style);

    fabButton.addEventListener('click', (e) => {
        e.stopPropagation();
        e.preventDefault();
        createEmailTailorModal();
    });

    document.body.appendChild(fabButton);
}

const observer = new MutationObserver((mutationsList, observerInstance) => {
    const nameLoaded = document.querySelector('h1.text-heading-xlarge, li.inline.t-24.v-align-middle');
    if (nameLoaded) { 
        injectFloatingMailButton();
    }
});

observer.observe(document.body, { childList: true, subtree: true });

setTimeout(injectFloatingMailButton, 1500);
setTimeout(injectFloatingMailButton, 3500);

// --- Message Listener for API calls from popup ---
chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
  if (request.type === 'GENERATE_EMAIL') {
    const linkedin_url = window.location.href;
    const user_prompt = request.context;

    if (!user_prompt) {
      sendResponse({ error: "User prompt is missing." });
      return false; // No async response needed
    }

    // Indicate that we will send a response asynchronously
    // This is crucial for keeping the message channel open
    (async () => {
      try {
        const apiResponse = await fetch('http://localhost:3000/generate-email', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({ linkedin_url, user_prompt }),
        });

        if (!apiResponse.ok) {
          let errorMessage = `API Error: ${apiResponse.status}`;
          try {
            const errorData = await apiResponse.json();
            errorMessage = errorData.message || errorData.error || `API Error ${apiResponse.status}: ${apiResponse.statusText}`;
          } catch (e) {
            // If error response is not JSON, use status text
            errorMessage = `API Error ${apiResponse.status}: ${apiResponse.statusText || 'Failed to fetch'}`;
          }
          sendResponse({ error: errorMessage });
          return;
        }

        const data = await apiResponse.json();
        if (data.generated_email) {
          sendResponse({ email: data.generated_email });
        } else if (data.error) { // Handle specific error from backend if provided
          sendResponse({ error: data.error });
        } else {
          sendResponse({ error: "Received no email content or error from the backend." });
        }
      } catch (error) {
        console.error('ContentScript - Fetch API call failed:', error);
        sendResponse({ error: `Network error: ${error.message}. Ensure backend is running.` });
      }
    })();

    return true; // Required to indicate an asynchronous response.
  }
  return false; // For other message types, if any in the future.
});

/*
// --- Original Message Listener for Popup --- 
chrome.runtime.onMessage.addListener((msg, sender, sendResponse) => {
    if (msg.type === 'GENERATE_EMAIL') { 
        const context = msg.context;
        const profileData = scrapeLinkedInProfile(); // Assuming this function exists and is what you want

        if (profileData.error) {
            sendResponse({ error: profileData.error });
            return;
        }

        // Placeholder: Simulate AI call or integrate actual AI model here
        // For now, just echo back the context and some dummy profile data
        setTimeout(() => {
            const generatedEmail = `Email for ${profileData.name || 'Profile'}:\nContext: ${context}\nLocation: ${profileData.location || 'N/A'}\nHeadline: ${profileData.headline || 'N/A'}`;
            sendResponse({ email: generatedEmail });
        }, 1000);

        return true; // Indicates an asynchronous response.
    }
});
*/