import React from 'react';
import './AppChatGPT.css';

function App() {
  return (
    <div className="chatgpt-app">
      <aside className="sidebar">
        <div className="sidebar-header">ChatGPT</div>
        <ul className="conversation-list">
          <li className="conversation-item active">今天</li>
          <li className="conversation-item">前 30 天</li>
          <li className="conversation-item">四月</li>
        </ul>
      </aside>
      <main className="main-content">
        <div className="chat-history">
          <div className="message user">你好！</div>
          <div className="message assistant">你好，有什么可以帮您？</div>
        </div>
        <div className="chat-input-wrapper">
          <input className="chat-input" type="text" placeholder="输入你的问题..." />
          <button className="send-btn">发送</button>
        </div>
      </main>
    </div>
  );
}

export default App;
