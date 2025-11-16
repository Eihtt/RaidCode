import React, { useState } from 'react'; // For GitHub Pages, use CDN or static

function RaidDashboard() {
  const [wallet, setWallet] = useState(null);
  const bounties = [
    { id: 1, title: "NYSE Pump Bot", desc: "Build S&P predictor – US traders love it", reward: 1000 },
    { id: 2, title: "GitHub Meme Fork", desc: "Add viral Twitter script", reward: 500 }
  ];

  const connectWallet = async () => {
    if (window.solana) {
      const resp = await window.solana.connect();
      setWallet(resp.publicKey.toString());
      alert('Connected! Pick a bounty and fork.');
    }
  };

  return (
    <div style={{ background: '#000', color: '#00ff41', padding: '20px', fontFamily: 'monospace' }}>
      <h1>RaidCode US Dashboard</h1>
      {!wallet ? <button onClick={connectWallet}>Connect Phantom</button> : <p>Wallet: {wallet.slice(0,8)}...</p>}
      <h2>Open US Bounties</h2>
      <ul>
        {bounties.map(b => (
          <li key={b.id}>
            <strong>{b.title}</strong>: {b.desc} – Reward: {b.reward} $RAID
            <button>Fork & Raid</button>
          </li>
        ))}
      </ul>
    </div>
  );
}

// For static HTML: Embed as script in index.html

