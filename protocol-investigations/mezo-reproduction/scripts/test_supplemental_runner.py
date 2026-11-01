"""Offline parser/privacy controls only. These tests do not execute Mezo."""
import unittest
import reproduce_supplemental as runner


def transcript():
    lines = ["[PASS] " + n + "()" for n in sorted(runner.NAMES)]
    lines = [x + " (runs: 1024)" if "testFuzzMaterialQuotesAllPairs" in x else x for x in lines]
    return "\n".join(lines + ["Suite result: ok. 9 passed; 0 failed; 0 skipped"])


class ParserControls(unittest.TestCase):
    def test_accepts_complete_named_record(self):
        self.assertTrue(runner.validate_output(transcript(), 0)["validated_pass"])

    def test_rejects_zero_tests(self):
        self.assertFalse(runner.validate_output("No tests found", 0)["validated_pass"])

    def test_rejects_wrong_name(self):
        self.assertFalse(runner.validate_output(transcript().replace("testQuoteGridCoverage", "testOther"), 0)["validated_pass"])

    def test_rejects_missing_fuzz_count(self):
        self.assertFalse(runner.validate_output(transcript().replace("runs: 1024", "runs: 12"), 0)["validated_pass"])

    def test_rejects_failed_exit_even_with_pass_lines(self):
        self.assertFalse(runner.validate_output(transcript(), 1)["validated_pass"])

    def test_rejects_skipped_summary(self):
        self.assertFalse(runner.validate_output(transcript().replace("0 skipped", "1 skipped"), 0)["validated_pass"])

    def test_rejects_duplicate_pass_name(self):
        self.assertFalse(runner.validate_output(transcript()+"\n[PASS] testQuoteGridCoverage()", 0)["validated_pass"])

    def test_redacts_endpoint_and_personal_prefix(self):
        secret = "https://example.invalid/private-token"
        text = runner.clean(secret+" C:/Users/Example/report", secret)
        self.assertNotIn("private-token", text)
        self.assertNotIn("Example", text)


if __name__ == "__main__":
    unittest.main()
