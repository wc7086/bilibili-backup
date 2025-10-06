/**
 * App 根组件
 *
 * 配置路由和全局布局
 */

import { HashRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import Layout from './components/Layout';
import Login from './pages/Login';
import Dashboard from './pages/Dashboard';
import Following from './pages/Following';
import Followers from './pages/Followers';
import Blacklist from './pages/Blacklist';
import Favorites from './pages/Favorites';
import History from './pages/History';
import Bangumi from './pages/Bangumi';
import ToView from './pages/ToView';
import './App.css';

function App() {
  return (
    <Router>
      <Routes>
        <Route path="/login" element={<Login />} />
        <Route path="/" element={<Layout />}>
          <Route index element={<Navigate to="/dashboard" replace />} />
          <Route path="dashboard" element={<Dashboard />} />
          <Route path="following" element={<Following />} />
          <Route path="followers" element={<Followers />} />
          <Route path="blacklist" element={<Blacklist />} />
          <Route path="favorites" element={<Favorites />} />
          <Route path="history" element={<History />} />
          <Route path="bangumi" element={<Bangumi />} />
          <Route path="toview" element={<ToView />} />
        </Route>
      </Routes>
    </Router>
  );
}

export default App;
