import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.maximizeWindow()

WebUI.navigateToUrl(GlobalVariable.dev)

WebUI.delay(3)

WebUI.sendKeys(findTestObject('CMS/Login/Label_Username'), Keys.chord(Keys.SHIFT, Keys.ARROW_UP, Keys.DELETE))

WebUI.setText(findTestObject('CMS/Login/Label_Username'), 'rudi@member.id')

WebUI.sendKeys(findTestObject('CMS/Login/Label_Password'), Keys.chord(Keys.SHIFT, Keys.ARROW_UP))

WebUI.sendKeys(findTestObject('CMS/Login/Label_Password'), Keys.chord(Keys.DELETE))

WebUI.setText(findTestObject('CMS/Login/Label_Password'), 'memberid123')

WebUI.click(findTestObject('CMS/Login/Btn_SignIn'))

WebUI.delay(2)

