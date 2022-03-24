import React, { useEffect, useState } from "react"
import './App.css'
import { Tabs, Button, List, Card } from "antd";
import { login, logout } from "./utils";

const { TabPane } = Tabs;

function App() {
  const [isSignIn, setIsSignIn] = useState(false)

  useEffect(() => {
    if (window.walletConnection.isSignedIn()) {
      setIsSignIn(true)
    }
  }, [])
  const signInButton = <Button onClick={() => login()}>Sign In</Button>
  const signOutButton = <Button onClick={() => logout()}>Sign Out</Button>;
  return (
    <Tabs tabBarExtraContent={isSignIn ? signOutButton : signInButton} style={{padding: '10px 20px', margin: '20px', border: 'dashed 1px'}}>
      <TabPane tab="Farm List" key="1">
        <FarmList/>
      </TabPane>
      <TabPane tab="Create Farm" key="2">
        Content of tab 2
      </TabPane>
    </Tabs>
  );
}

const FarmList = () => {
  const [list, setList] = useState([])

  useEffect(() => {
    if (window.farmingContract) {
      window.farmingContract
        .list_farms({ from_index: 0, limit: 10 })
        .then((farms) => {
          if (farms && farms.length) {
            setList(farms);
          }
        });
    }
  }, []);

  return (
    <List
      grid={{ gutter: 16, column: 4 }}
      dataSource={list}
      renderItem={(item) => (
        <List.Item>
          <Card title={item.farm_id}>
            <p>Seed Reward: {item.seed_id}</p>
            <p>Remain: {item.total_reward}</p>
            <p>Claimed: {item.claimed_reward}</p>
            <p>Start at: {item.start_at}</p>
            <p>Reward per session: {item.reward_per_session}</p>
            <p>Status: {item.farm_status}</p>
          </Card>
        </List.Item>
      )}
    />
  );
}

export default App
