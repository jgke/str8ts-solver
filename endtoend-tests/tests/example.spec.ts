import { test, expect } from '@playwright/test';

test('test', async ({ page }) => {
  await page.goto('http://localhost:8080/');
  await page.getByLabel(/Enter the puzzle/).click();
  await page.getByLabel(/Enter the puzzle/).fill('#.......5\n.5..#...7\n2.4.7.65.\n..#.#....\nd#.8b6.#a\n....#.#..\n.75.6.3.4\n7...#..3.\n6.......#\n');
  await page.getByRole('button', { name: 'Parse' }).click();
  await page.locator('label').filter({ hasText: 'Open menu' }).click();
  await page.getByRole('button', { name: 'Solve', exact: true }).click();
  await expect(page.locator('css=#puzzle-rating-stars>*')).toHaveCount(4);
});
