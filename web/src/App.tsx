import Navbar from "./components/Navbar";
import Hero from "./components/Hero";
import TechnicalWorkflow from "./components/TechnicalWorkflow";
import APIPlayground from "./components/APIPlayground";
import Implementation from "./components/Implementation";
import SecuritySection from "./components/SecuritySection";
import Footer from "./components/Footer";

const App: React.FC = () => {
  return (
    <div className="min-h-screen bg-iris-black text-white selection:bg-iris-purple selection:text-iris-black">
      <Navbar />
      <Hero />
      <TechnicalWorkflow />
      <APIPlayground />
      <Implementation />
      <SecuritySection />
      <Footer />
    </div>
  );
};

export default App;
