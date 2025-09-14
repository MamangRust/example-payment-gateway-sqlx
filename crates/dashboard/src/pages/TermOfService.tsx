import { Link } from "react-router-dom";

export default function TermsOfServicePage() {
  return (
    <div className="container mx-auto px-4 py-12">
      <h1 className="text-3xl font-bold mb-6">Terms of Service</h1>

      <p className="mb-4">
        Welcome to [Your App Name]! These terms and conditions outline the rules
        and regulations for the use of [Your Company Name]'s website and
        services, located at [Your Domain].
      </p>

      <h2 className="text-2xl font-semibold mb-3">1. Acceptance of Terms</h2>
      <p className="mb-4">
        By accessing this website, we assume you accept these terms and
        conditions. Do not continue to use [Your App Name] if you do not agree
        to all the terms and conditions stated on this page.
      </p>

      <h2 className="text-2xl font-semibold mb-3">2. Privacy Policy</h2>
      <p className="mb-4">
        Your use of [Your App Name] is also governed by our{" "}
        <Link to="/privacy" className="underline hover:text-primary">
          Privacy Policy
        </Link>
        , which explains how we collect, use, and protect your information.
      </p>

      <h2 className="text-2xl font-semibold mb-3">3. License</h2>
      <p className="mb-4">
        Unless otherwise stated, [Your Company Name] and/or its licensors own
        the intellectual property rights for all material on [Your App Name].
        All intellectual property rights are reserved. You may access this from
        [Your App Name] for your own personal use, subjected to restrictions set
        in these terms and conditions.
      </p>

      <h2 className="text-2xl font-semibold mb-3">4. User Content</h2>
      <p className="mb-4">
        You are responsible for any content you submit to our platform. By
        posting content, you warrant that you have the right to do so and grant
        us a non-exclusive, worldwide, royalty-free license to use, modify, and
        distribute that content in connection with our services.
      </p>

      <h2 className="text-2xl font-semibold mb-3">5. Prohibited Activities</h2>
      <ul className="list-disc list-inside mb-4">
        <li>Engaging in unlawful activities using our platform.</li>
        <li>Impersonating another person or entity.</li>
        <li>Spamming or sending unsolicited messages.</li>
        <li>Uploading or distributing viruses or malicious software.</li>
      </ul>

      <h2 className="text-2xl font-semibold mb-3">6. Limitation of Liability</h2>
      <p className="mb-4">
        To the maximum extent permitted by applicable law, [Your Company Name]
        shall not be liable for any damages arising out of or in connection with
        your use of the website or services. This includes, without limitation,
        direct, indirect, incidental, punitive, and consequential damages.
      </p>

      <h2 className="text-2xl font-semibold mb-3">7. Changes to the Terms</h2>
      <p className="mb-4">
        We reserve the right to modify these terms at any time. If we make
        changes, we will post the updated terms on this page and update the
        effective date at the top of the page. Your continued use of our
        services constitutes your acceptance of the new terms.
      </p>

      <h2 className="text-2xl font-semibold mb-3">8. Contact Us</h2>
      <p className="mb-4">
        If you have any questions about these Terms, please contact us at{" "}
        <a href="mailto:support@yourcompany.com" className="underline">
          support@yourcompany.com
        </a>
        .
      </p>

      <Link to="/" className="underline hover:text-primary">
        Return to Home
      </Link>
    </div>
  );
}
