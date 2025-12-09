import { useSpring, animated } from '@react-spring/web';
import { Wallet } from 'lucide-react';
import { useEffect, useState } from 'react';

const SplashScreen = () => {
  const [isDarkMode, setIsDarkMode] = useState(false);

  useEffect(() => {
    const root = document.documentElement;
    setIsDarkMode(root.classList.contains('dark'));
  }, []);

  const fadeProps = useSpring({
    opacity: 1,
    from: { opacity: 0 },
    config: { duration: 1000 },
  });
  const scaleProps = useSpring({
    transform: 'scale(1)',
    from: { transform: 'scale(0.5)' },
    config: { duration: 1000 },
  });
  const slideProps = useSpring({
    transform: 'translateY(0px)',
    from: { transform: 'translateY(50px)' },
    config: { duration: 1000, delay: 1000 },
  });

  return (
    <div
      className={`flex justify-center items-center min-h-screen ${
        isDarkMode
          ? 'bg-gradient-to-br from-gray-800 to-gray-900'
          : 'bg-gradient-to-br from-blue-400 to-teal-600'
      }`}
    >
      <div className="relative">
        <animated.div
          style={{
            transform: scaleProps.transform.to((t) => `${t} rotate(2deg)`),
          }}
          className={`absolute top-0 left-0 w-40 h-40 rounded-full ${
            isDarkMode ? 'bg-gray-700 opacity-10' : 'bg-white opacity-10'
          }`}
        ></animated.div>
        <animated.div
          style={{
            transform: scaleProps.transform.to((t) => `${t} rotate(-2deg)`),
          }}
          className={`absolute bottom-0 right-0 w-60 h-60 rounded-full ${
            isDarkMode ? 'bg-gray-700 opacity-10' : 'bg-white opacity-10'
          }`}
        ></animated.div>

        {/* Main Content */}
        <div className="relative z-10 flex flex-col items-center">
          <animated.div style={scaleProps}>
            <animated.div
              style={fadeProps}
              className={`w-30 h-30 rounded-lg shadow-lg flex items-center justify-center ${
                isDarkMode ? 'bg-gray-700' : 'bg-white'
              }`}
            >
              <Wallet
                className={`w-16 h-16 ${
                  isDarkMode ? 'text-gray-300' : 'text-teal-600'
                }`}
              />
            </animated.div>
          </animated.div>
          <div className="mt-6">
            <animated.div style={slideProps}>
              <animated.div
                style={fadeProps}
                className={`text-4xl font-bold ${
                  isDarkMode ? 'text-gray-300' : 'text-white'
                }`}
              >
                SanEdge
              </animated.div>
            </animated.div>
            <animated.div style={slideProps}>
              <animated.div
                style={fadeProps}
                className={`text-lg opacity-80 ${
                  isDarkMode ? 'text-gray-400' : 'text-white'
                }`}
              >
                Your Digital Wallet Solution
              </animated.div>
            </animated.div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default SplashScreen;
