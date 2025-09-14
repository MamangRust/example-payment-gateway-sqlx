import { Link } from 'react-router-dom';
import {
  ChevronRight,
  CreditCard,
  Shield,
  Globe,
  Zap,
  BarChart,
} from 'lucide-react';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { Button } from '@/components/ui/button';
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  CardDescription,
  CardFooter,
} from '@/components/ui/card';
import { Navbar } from '@/components/home/SiteHeader';

export default function CompanyProfile() {
  return (
    <div className="flex flex-col min-h-screen dark:bg-gray-900">
      <Navbar />
      <main className="flex-1">
        <section className="w-full py-12 md:py-24 lg:py-32 xl:py-48 bg-gradient-to-r from-blue-600 to-blue-800 dark:from-blue-800 dark:to-blue-950">
          <div className="container mx-auto px-4 md:px-6">
            <div className="flex flex-col items-center space-y-4 text-center">
              <div className="space-y-2">
                <h1 className="text-3xl font-bold tracking-tighter sm:text-4xl md:text-5xl lg:text-6xl/none text-white">
                  Welcome to PaySanEdge
                </h1>
                <p className="mx-auto max-w-[700px] text-gray-200 md:text-xl">
                  Simplifying payments for businesses worldwide with secure and
                  innovative solutions.
                </p>
              </div>
              <div className="space-x-4">
                <Button variant={'outline'} className="text-blue-600">
                  Our Solutions
                </Button>
                <Button variant="outline" className="text-blue-600">
                  Get Started
                </Button>
              </div>
            </div>
          </div>
        </section>
        <section className="w-full py-12 md:py-24 lg:py-32 bg-gray-50 dark:bg-gray-800">
          <div className="container mx-auto px-4 md:px-6">
            <h2 className="text-3xl font-bold tracking-tighter sm:text-5xl text-center mb-12 text-blue-600">
              Our Payment Solutions
            </h2>
            <div className="grid gap-6 lg:grid-cols-3 lg:gap-12 justify-items-center">
              {[
                {
                  title: 'Online Payments',
                  description:
                    'Secure and seamless online transaction processing',
                  icon: <CreditCard className="h-6 w-6 text-blue-600" />,
                },
                {
                  title: 'Mobile Payments',
                  description:
                    'Convenient payment solutions for on-the-go transactions',
                  icon: <Zap className="h-6 w-6 text-blue-600" />,
                },
                {
                  title: 'Global Transactions',
                  description:
                    'Support for multiple currencies and international payments',
                  icon: <Globe className="h-6 w-6 text-blue-600" />,
                },
              ].map((service, i) => (
                <Card
                  key={i}
                  className="border-blue-200 dark:border-blue-800 hover:border-blue-400 dark:hover:border-blue-600 transition-colors duration-300 dark:bg-gray-800"
                >
                  <CardHeader>
                    <CardTitle className="flex items-center text-blue-600">
                      {service.icon}
                      <span className="ml-2">{service.title}</span>
                    </CardTitle>
                    <CardDescription>{service.description}</CardDescription>
                  </CardHeader>
                  <CardContent>
                    <p>
                      Our {service.title.toLowerCase()} solution provides
                      businesses with a robust platform for seamless
                      transactions...
                    </p>
                  </CardContent>
                  <CardFooter>
                    <Button variant="ghost" className="text-blue-600">
                      Learn More
                    </Button>
                  </CardFooter>
                </Card>
              ))}
            </div>
          </div>
        </section>
        <section className="w-full py-12 md:py-24 lg:py-32 dark:bg-gray-900">
          <div className="container mx-auto px-4 md:px-6">
            <h2 className="text-3xl font-bold tracking-tighter sm:text-5xl text-center mb-12 text-blue-600">
              Why Choose PayEase
            </h2>
            <div className="max-w-3xl mx-auto">
              <div className="space-y-8">
                {[
                  {
                    title: 'Security First',
                    description:
                      'Bank-grade encryption and fraud protection to keep your transactions safe.',
                    icon: <Shield className="h-6 w-6 text-blue-600" />,
                  },
                  {
                    title: 'Global Reach',
                    description:
                      'Accept payments from customers worldwide in multiple currencies.',
                    icon: <Globe className="h-6 w-6 text-blue-600" />,
                  },
                  {
                    title: 'Easy Integration',
                    description:
                      'Simple API and plugins for quick integration with your existing systems.',
                    icon: <Zap className="h-6 w-6 text-blue-600" />,
                  },
                  {
                    title: 'Competitive Rates',
                    description:
                      'Transparent pricing with some of the lowest transaction fees in the industry.',
                    icon: <CreditCard className="h-6 w-6 text-blue-600" />,
                  },
                  {
                    title: 'Analytics Dashboard',
                    description:
                      'Real-time insights into your payment data to help grow your business.',
                    icon: <BarChart className="h-6 w-6 text-blue-600" />,
                  },
                ].map((item, i) => (
                  <article
                    key={i}
                    className="flex flex-col gap-4 md:flex-row md:items-start"
                  >
                    <div className="flex h-12 w-12 items-center justify-center rounded-lg bg-blue-100 text-blue-600">
                      {item.icon}
                    </div>
                    <div className="flex flex-col gap-2">
                      <h3 className="text-2xl font-bold text-blue-600">
                        {item.title}
                      </h3>
                      <p className="text-gray-600 dark:text-gray-300">
                        {item.description}
                      </p>
                      <Button
                        variant="link"
                        className="w-fit p-0 text-blue-600"
                      >
                        Learn More <ChevronRight className="ml-2 h-4 w-4" />
                      </Button>
                    </div>
                  </article>
                ))}
              </div>
            </div>
          </div>
        </section>
        <section className="w-full py-12 md:py-24 lg:py-32 bg-gray-50 dark:bg-gray-800">
          <div className="container mx-auto px-4 md:px-6">
            <h2 className="text-3xl font-bold tracking-tighter sm:text-5xl text-center mb-12 text-blue-600">
              Our Leadership Team
            </h2>
            <div className="grid gap-6 lg:grid-cols-3 lg:gap-12 justify-items-center">
              {[
                {
                  name: 'Sarah Johnson',
                  role: 'CEO & Founder',
                  bio: 'With 15 years in fintech, Sarah leads PayEase with a vision for global, secure payments.',
                },
                {
                  name: 'Michael Chen',
                  role: 'CTO',
                  bio: 'A cybersecurity expert, Michael ensures our platform stays at the forefront of payment security.',
                },
                {
                  name: 'Emily Rodriguez',
                  role: 'Head of Global Partnerships',
                  bio: "Emily expands PayEase's reach, fostering relationships with businesses worldwide.",
                },
              ].map((member, i) => (
                <Card
                  key={i}
                  className="border-blue-200 dark:border-blue-800 hover:border-blue-400 dark:hover:border-blue-600 transition-colors duration-300 dark:bg-gray-800"
                >
                  <CardHeader>
                    <Avatar className="h-24 w-24 mx-auto mb-4">
                      <AvatarImage
                        src={`/placeholder.svg?height=96&width=96&text=${member.name.charAt(
                          0
                        )}`}
                        alt={member.name}
                      />
                      <AvatarFallback>{member.name.charAt(0)}</AvatarFallback>
                    </Avatar>
                    <CardTitle className="text-blue-600">
                      {member.name}
                    </CardTitle>
                    <CardDescription>{member.role}</CardDescription>
                  </CardHeader>
                  <CardContent>
                    <p>{member.bio}</p>
                  </CardContent>
                </Card>
              ))}
            </div>
          </div>
        </section>
      </main>
      <footer className="w-full border-t border-blue-200 dark:border-blue-800 bg-gray-50 dark:bg-gray-900 py-6">
        <div className="container mx-auto flex flex-col items-center justify-center gap-4 md:h-24 md:flex-row md:justify-between">
          <p className="text-center text-sm leading-loose text-gray-600 dark:text-gray-300 md:text-left">
            © 2023 PayEase. All rights reserved.
          </p>
          <div className="flex items-center space-x-4">
            <Link
              className="text-sm text-blue-600 hover:underline underline-offset-4"
              to="#"
            >
              Privacy Policy
            </Link>
            <Link
              className="text-sm text-blue-600 hover:underline underline-offset-4"
              to="#"
            >
              Terms of Service
            </Link>
            <Link
              className="text-sm text-blue-600 hover:underline underline-offset-4"
              to="#"
            >
              Contact Us
            </Link>
          </div>
        </div>
      </footer>
    </div>
  );
}
