import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

function RecordCustomEndpoint() {
  const [formData, setFormData] = useState({
    baseUrl: "",
    platformId: "",
    gameId: "",
    encryptionKey: "",
  })

  async function record() {
    console.log(formData)
    await invoke("record_custom_endpoint", formData)
  }

  return (
    <form
      className="row"
      onSubmit={(e) => {
        e.preventDefault();
        record();
      }}
    >
      <input
        id="base-url"
        onChange={(e) => setFormData({...formData, baseUrl: e.currentTarget.value})}
        placeholder="Enter base url"
      />
      <input
        id="platform-id"
        onChange={(e) => setFormData({...formData, platformId: e.currentTarget.value})}
        placeholder="Enter platform id"
      />
      <input
        id="game-id"
        onChange={(e) => setFormData({...formData, gameId: e.currentTarget.value})}
        placeholder="Enter game id"
      />
      <input
        id="encryption-key"
        onChange={(e) => setFormData({...formData, encryptionKey: e.currentTarget.value})}
        placeholder="Enter encryption key"
      />
      <button type="submit">Record</button>
    </form>
  )
}

export default RecordCustomEndpoint
