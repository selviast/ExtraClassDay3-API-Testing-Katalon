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
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.testobject.ResponseObject
import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

// Kirim request PUT /pet
ResponseObject response = WS.sendRequest(findTestObject('Petstore/Update Pet'))

// Assertion: status code = 200
WS.verifyResponseStatusCode(response, 200)

// Parse response JSON
def json = new JsonSlurper().parseText(response.getResponseBodyContent())

// Debug output
println("Response Body: " + response.getResponseBodyContent())
println("Updated Pet ID: " + json.id)
println("Updated Pet Name: " + json.name)
println("Updated Pet Status: " + json.status)

// Assertion: validasi fields yang diupdate
WS.verifyElementPropertyValue(response, 'id', 12345)   // pastikan ID sama
WS.verifyElementPropertyValue(response, 'name', 'Selviaa')       // ganti sesuai data update
WS.verifyElementPropertyValue(response, 'status', 'sold')            // ganti sesuai data update


WS.sendRequest(findTestObject('Petstore/Update Pet'))



